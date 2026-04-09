//! Rust binding generation from C headers.

use std::path::{Path, PathBuf};

/// Maximum header file size accepted for binding generation (denial-of-service guard).
const MAX_HEADER_BYTES: u64 = 10 * 1024 * 1024;

/// Maximum number of lines parsed from a header (CPU / memory guard).
const MAX_HEADER_LINES: usize = 200_000;

/// Options for binding generation.
#[derive(Clone, Debug, Default)]
pub struct BindingOptions {
    /// Module name for the generated bindings.
    pub module_name: Option<String>,
    /// Additional include paths.
    pub include_paths: Vec<PathBuf>,
    /// Functions to allowlist (if empty, include all).
    pub allowlist_functions: Vec<String>,
    /// Types to allowlist (if empty, include all).
    pub allowlist_types: Vec<String>,
    /// Generate impl blocks for types.
    pub derive_debug: bool,
    /// Generate Default impl.
    pub derive_default: bool,
}

/// A generated Rust binding.
#[derive(Clone, Debug)]
pub struct GeneratedBinding {
    /// The generated Rust code.
    pub code: String,
    /// The source header file.
    pub source_header: PathBuf,
    /// Any warnings during generation.
    pub warnings: Vec<String>,
}

/// Generate Rust bindings from a C header file.
///
/// This creates a Rust module with extern "C" declarations
/// that can be used to call the compiled C code.
pub fn generate_bindings(
    header: &Path,
    options: &BindingOptions,
) -> Result<GeneratedBinding, String> {
    if !header.exists() {
        return Err(format!("Header file not found: {}", header.display()));
    }
    if !header.is_file() {
        return Err(format!(
            "Header path is not a regular file: {}",
            header.display()
        ));
    }

    let meta =
        std::fs::metadata(header).map_err(|e| format!("Failed to read header metadata: {e}"))?;
    if meta.len() > MAX_HEADER_BYTES {
        return Err(format!(
            "Header too large ({} bytes; max {} bytes)",
            meta.len(),
            MAX_HEADER_BYTES
        ));
    }

    let content =
        std::fs::read_to_string(header).map_err(|e| format!("Failed to read header: {e}"))?;

    let line_count = content.lines().count();
    if line_count > MAX_HEADER_LINES {
        return Err(format!(
            "Header has too many lines ({line_count}; max {MAX_HEADER_LINES})"
        ));
    }

    let mut warnings = Vec::new();
    let mut code = String::new();

    // File header — use regular // comments so the output is valid when include!()'d into a module
    code.push_str("// Auto-generated bindings by equilibrium-ffi\n");
    code.push_str("//\n");
    code.push_str(&format!(
        "// Source: {}\n",
        sanitize_path_for_comment(header)
    ));
    code.push('\n');
    code.push_str("use std::os::raw::*;\n");
    code.push('\n');

    // Parse and generate bindings
    let parsed = parse_c_header(&content);

    // Generate enum definitions
    for enum_def in &parsed.enums {
        if should_include(&enum_def.name, &options.allowlist_types) {
            code.push_str(&generate_enum(enum_def, options));
            code.push('\n');
        }
    }

    // Generate struct definitions
    for struct_def in &parsed.structs {
        if should_include(&struct_def.name, &options.allowlist_types) {
            code.push_str(&generate_struct(struct_def, options));
            code.push('\n');
        }
    }

    // Generate type definitions (but skip typedef struct/enum aliases since we already have them)
    for typedef in &parsed.typedefs {
        if should_include(&typedef.name, &options.allowlist_types) {
            // Skip if this is just an alias to a struct/enum we already generated
            let is_struct_alias = typedef.target.starts_with("struct ")
                && parsed
                    .structs
                    .iter()
                    .any(|s| format!("struct {}", s.name) == typedef.target);
            let is_enum_alias = typedef.target.starts_with("enum ")
                && parsed
                    .enums
                    .iter()
                    .any(|e| format!("enum {}", e.name) == typedef.target);
            if !is_struct_alias && !is_enum_alias {
                code.push_str(&generate_typedef(typedef, options));
                code.push('\n');
            }
        }
    }

    // Generate extern block with functions
    code.push_str("#[allow(non_camel_case_types, non_snake_case, dead_code)]\n");
    code.push_str("extern \"C\" {\n");
    for func in &parsed.functions {
        if should_include(&func.name, &options.allowlist_functions) {
            code.push_str(&generate_function(func));
        } else {
            warnings.push(format!("Skipped function: {}", func.name));
        }
    }
    code.push_str("}\n");

    Ok(GeneratedBinding {
        code,
        source_header: header.to_path_buf(),
        warnings,
    })
}

fn should_include(name: &str, allowlist: &[String]) -> bool {
    allowlist.is_empty() || allowlist.iter().any(|a| a == name)
}

/// Strip control characters from a path so a generated `//` comment stays single-line.
fn sanitize_path_for_comment(path: &Path) -> String {
    path.display()
        .to_string()
        .chars()
        .filter(|c| !matches!(c, '\n' | '\r' | '\0'))
        .collect()
}

// Simple C header parsing (for basic cases)
// In production, you'd use bindgen or a proper C parser

struct ParsedHeader {
    typedefs: Vec<TypedefDef>,
    structs: Vec<StructDef>,
    enums: Vec<EnumDef>,
    functions: Vec<FunctionDef>,
}

struct TypedefDef {
    name: String,
    target: String,
}

struct StructDef {
    name: String,
    fields: Vec<(String, String)>,
}

struct EnumDef {
    name: String,
    variants: Vec<(String, Option<String>)>, // (name, optional value)
}

struct FunctionDef {
    name: String,
    return_type: String,
    params: Vec<(String, String)>,
}

fn parse_c_header(content: &str) -> ParsedHeader {
    // Stop scanning a typedef block after this many lines without `}` (corrupt / hostile input).
    const MAX_TYPEDEF_BLOCK_LINES: usize = 16_384;

    let mut typedefs = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut functions = Vec::new();

    let mut i = 0;
    let lines: Vec<&str> = content.lines().collect();

    while i < lines.len() {
        let line = lines[i].trim();

        // Simple typedef detection
        if line.starts_with("typedef") {
            // Check if it's a typedef enum
            if line.contains("enum") && line.contains('{') {
                // Multi-line typedef enum
                let mut enum_content = String::new();
                let mut extend_lines = 0usize;
                while i < lines.len() && !lines[i].contains('}') {
                    extend_lines += 1;
                    if extend_lines > MAX_TYPEDEF_BLOCK_LINES {
                        break;
                    }
                    enum_content.push_str(lines[i]);
                    enum_content.push(' ');
                    i += 1;
                }
                if i < lines.len() && lines[i].contains('}') {
                    enum_content.push_str(lines[i]);
                }
                // Parse typedef enum
                if let Some(parsed) = parse_typedef_enum(&enum_content) {
                    typedefs.push(TypedefDef {
                        name: parsed.name.clone(),
                        target: format!("enum {}", parsed.name),
                    });
                    enums.push(parsed);
                }
            } else if line.contains("struct") && line.contains('{') {
                // Multi-line typedef struct
                let mut struct_content = String::new();
                let mut extend_lines = 0usize;
                while i < lines.len() && !lines[i].contains('}') {
                    extend_lines += 1;
                    if extend_lines > MAX_TYPEDEF_BLOCK_LINES {
                        break;
                    }
                    struct_content.push_str(lines[i]);
                    struct_content.push(' ');
                    i += 1;
                }
                if i < lines.len() && lines[i].contains('}') {
                    struct_content.push_str(lines[i]);
                }
                // Parse typedef struct
                if let Some(parsed) = parse_typedef_struct(&struct_content) {
                    typedefs.push(TypedefDef {
                        name: parsed.name.clone(),
                        target: format!("struct {}", parsed.name),
                    });
                    structs.push(parsed);
                }
            } else if line.ends_with(';') {
                // Simple typedef
                if let Some((target, name)) = parse_typedef_line(line) {
                    typedefs.push(TypedefDef { name, target });
                }
            }
        }

        // Simple function declaration detection
        if !line.starts_with("typedef")
            && !line.starts_with("struct")
            && !line.starts_with("enum")
            && !line.starts_with("//")
            && !line.starts_with("#")
            && line.contains('(')
            && line.ends_with(';')
        {
            if let Some(func) = parse_function_line(line) {
                functions.push(func);
            }
        }

        i += 1;
    }

    ParsedHeader {
        typedefs,
        structs,
        enums,
        functions,
    }
}

fn parse_typedef_struct(content: &str) -> Option<StructDef> {
    // typedef struct { ... } name;
    let content = content.trim();

    // Extract struct name from end: } NAME;
    let end_part = content.strip_suffix(';')?.trim();
    let name = end_part.split_whitespace().last()?.to_string();

    // Extract fields between { and }
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    let fields_str = &content[start + 1..end];

    let mut fields = Vec::new();
    // Split by semicolons, not lines (fields end with ;)
    for field in fields_str.split(';') {
        let field = field.trim();
        if field.is_empty() || field.starts_with("//") {
            continue;
        }

        // Parse "type name" or "type name[size]"
        // Handle "uint32_t Pin" or "char* name"
        let parts: Vec<&str> = field.split_whitespace().collect();
        if parts.len() >= 2 {
            let field_name = parts.last().unwrap().trim_end_matches('[').to_string();
            let field_type = parts[..parts.len() - 1].join(" ");
            fields.push((field_type, field_name));
        }
    }

    Some(StructDef { name, fields })
}

fn parse_typedef_enum(content: &str) -> Option<EnumDef> {
    // typedef enum { VAR1 = 0, VAR2 = 1 } name;
    let content = content.trim();

    // Extract enum name from end: } NAME;
    let end_part = content.strip_suffix(';')?.trim();
    let name = end_part.split_whitespace().last()?.to_string();

    // Extract variants between { and }
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    let variants_str = &content[start + 1..end];

    let mut variants = Vec::new();
    for item in variants_str.split(',') {
        let item = item.trim();
        if item.is_empty() || item.starts_with("//") {
            continue;
        }

        if let Some((name, value)) = item.split_once('=') {
            variants.push((name.trim().to_string(), Some(value.trim().to_string())));
        } else {
            variants.push((item.to_string(), None));
        }
    }

    Some(EnumDef { name, variants })
}

fn parse_typedef_line(line: &str) -> Option<(String, String)> {
    // typedef int myint;
    let line = line.strip_prefix("typedef")?.trim();
    let line = line.strip_suffix(';')?.trim();

    let parts: Vec<&str> = line.rsplitn(2, ' ').collect();
    if parts.len() == 2 {
        Some((parts[1].to_string(), parts[0].to_string()))
    } else {
        None
    }
}

fn parse_function_line(line: &str) -> Option<FunctionDef> {
    // int foo(int x, char *y);
    let line = line.strip_suffix(';')?.trim();

    let paren_start = line.find('(')?;
    let paren_end = line.rfind(')')?;

    let signature = &line[..paren_start].trim();
    let params_str = &line[paren_start + 1..paren_end];

    // Split return type and name
    let parts: Vec<&str> = signature.rsplitn(2, ' ').collect();
    let (return_type, name) = if parts.len() == 2 {
        (parts[1].to_string(), parts[0].to_string())
    } else {
        ("void".to_string(), parts[0].to_string())
    };

    // Parse parameters
    let params: Vec<(String, String)> = if params_str.trim() == "void" || params_str.is_empty() {
        Vec::new()
    } else {
        params_str
            .split(',')
            .filter_map(|p| {
                let p = p.trim();
                let parts: Vec<&str> = p.rsplitn(2, ' ').collect();
                if parts.len() == 2 {
                    // Handle `const char *name` — `*` may be stuck to the name.
                    // C notation puts `*` at the end of the type, e.g. `const char *`.
                    let (mut typ, mut name) = (parts[1].to_string(), parts[0].to_string());
                    if name.starts_with('*') {
                        let stars: String = name.chars().take_while(|&c| c == '*').collect();
                        name = name[stars.len()..].to_string();
                        typ = format!("{} {}", typ, stars);
                    }
                    Some((typ, name))
                } else {
                    None
                }
            })
            .collect()
    };

    Some(FunctionDef {
        name,
        return_type,
        params,
    })
}

fn generate_typedef(typedef: &TypedefDef, _options: &BindingOptions) -> String {
    let rust_type = c_type_to_rust(&typedef.target);
    format!("pub type {} = {};\n", typedef.name, rust_type)
}

fn generate_enum(enum_def: &EnumDef, _options: &BindingOptions) -> String {
    let mut code = String::new();

    code.push_str("#[repr(C)]\n");
    code.push_str("#[derive(Debug, Copy, Clone, PartialEq, Eq)]\n");
    code.push_str(&format!("pub enum {} {{\n", enum_def.name));

    for (variant_name, variant_value) in &enum_def.variants {
        if let Some(value) = variant_value {
            code.push_str(&format!("    {} = {},\n", variant_name, value));
        } else {
            code.push_str(&format!("    {},\n", variant_name));
        }
    }

    code.push_str("}\n");
    code
}

fn generate_struct(struct_def: &StructDef, options: &BindingOptions) -> String {
    let mut code = String::new();

    // Derive attributes
    let mut derives = vec!["Copy", "Clone"];
    if options.derive_debug {
        derives.push("Debug");
    }
    if options.derive_default {
        derives.push("Default");
    }

    code.push_str(&format!("#[derive({})]\n", derives.join(", ")));
    code.push_str("#[repr(C)]\n");
    code.push_str(&format!("pub struct {} {{\n", struct_def.name));

    for (field_type, field_name) in &struct_def.fields {
        let rust_type = c_type_to_rust(field_type);
        code.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    code.push_str("}\n");
    code
}

fn generate_function(func: &FunctionDef) -> String {
    let rust_return = c_type_to_rust(&func.return_type);

    let params: Vec<String> = func
        .params
        .iter()
        .map(|(typ, name)| format!("{}: {}", name, c_type_to_rust(typ)))
        .collect();

    let return_clause = if rust_return == "()" {
        String::new()
    } else {
        format!(" -> {}", rust_return)
    };

    format!(
        "    pub fn {}({}){};\n",
        func.name,
        params.join(", "),
        return_clause
    )
}

fn c_type_to_rust(c_type: &str) -> String {
    let c_type = c_type.trim();

    match c_type {
        "void" => "()".to_string(),
        "int" => "c_int".to_string(),
        "unsigned int" | "uint" => "c_uint".to_string(),
        "long" => "c_long".to_string(),
        "unsigned long" | "ulong" => "c_ulong".to_string(),
        "long long" => "c_longlong".to_string(),
        "unsigned long long" => "c_ulonglong".to_string(),
        "short" => "c_short".to_string(),
        "unsigned short" | "ushort" => "c_ushort".to_string(),
        "char" => "c_char".to_string(),
        "unsigned char" | "uchar" => "c_uchar".to_string(),
        "signed char" => "c_schar".to_string(),
        "float" => "c_float".to_string(),
        "double" => "c_double".to_string(),
        "size_t" => "usize".to_string(),
        "ssize_t" => "isize".to_string(),
        "bool" | "_Bool" => "bool".to_string(),
        // Standard C types from stdint.h
        "uint8_t" => "u8".to_string(),
        "uint16_t" => "u16".to_string(),
        "uint32_t" => "u32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int8_t" => "i8".to_string(),
        "int16_t" => "i16".to_string(),
        "int32_t" => "i32".to_string(),
        "int64_t" => "i64".to_string(),
        s if s.ends_with('*') => {
            let inner = s.strip_suffix('*').unwrap().trim();
            if inner == "void" {
                "*mut c_void".to_string()
            } else if inner == "const void" {
                "*const c_void".to_string()
            } else if inner.starts_with("const ") {
                let inner_type = c_type_to_rust(inner.strip_prefix("const ").unwrap());
                format!("*const {}", inner_type)
            } else {
                format!("*mut {}", c_type_to_rust(inner))
            }
        }
        s if s.starts_with("const ") => c_type_to_rust(s.strip_prefix("const ").unwrap()),
        other => other.to_string(), // Custom type, pass through
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_c_type_to_rust() {
        assert_eq!(c_type_to_rust("int"), "c_int");
        assert_eq!(c_type_to_rust("void"), "()");
        assert_eq!(c_type_to_rust("char *"), "*mut c_char");
        assert_eq!(c_type_to_rust("const char *"), "*const c_char");
    }

    #[test]
    fn test_c_type_to_rust_extended() {
        assert_eq!(c_type_to_rust("unsigned int"), "c_uint");
        assert_eq!(c_type_to_rust("unsigned long"), "c_ulong");
        assert_eq!(c_type_to_rust("long long"), "c_longlong");
        assert_eq!(c_type_to_rust("size_t"), "usize");
        assert_eq!(c_type_to_rust("ssize_t"), "isize");
        assert_eq!(c_type_to_rust("bool"), "bool");
        assert_eq!(c_type_to_rust("float"), "c_float");
        assert_eq!(c_type_to_rust("double"), "c_double");
        assert_eq!(c_type_to_rust("void*"), "*mut c_void");
        // const stripping
        assert_eq!(c_type_to_rust("const int"), "c_int");
    }

    #[test]
    fn test_parse_function() {
        let func = parse_function_line("int add(int a, int b);").unwrap();
        assert_eq!(func.name, "add");
        assert_eq!(func.return_type, "int");
        assert_eq!(func.params.len(), 2);
    }

    #[test]
    fn test_parse_function_void_params() {
        let func = parse_function_line("void cleanup(void);").unwrap();
        assert_eq!(func.name, "cleanup");
        assert_eq!(func.return_type, "void");
        assert_eq!(func.params.len(), 0);
    }

    #[test]
    fn test_parse_function_no_params() {
        let func = parse_function_line("int get_count();").unwrap();
        assert_eq!(func.name, "get_count");
        assert_eq!(func.return_type, "int");
        assert_eq!(func.params.len(), 0);
    }

    #[test]
    fn test_parse_function_pointer_param() {
        let func = parse_function_line("int string_length(const char* str);").unwrap();
        assert_eq!(func.name, "string_length");
        assert_eq!(func.return_type, "int");
        assert_eq!(func.params.len(), 1);
    }

    #[test]
    fn test_parse_typedef() {
        let (target, name) = parse_typedef_line("typedef int myint;").unwrap();
        assert_eq!(name, "myint");
        assert_eq!(target, "int");
    }

    #[test]
    fn test_parse_header_with_guards() {
        // Preprocessor directives should be ignored; typedefs inside guards should parse
        let content = "#ifndef MYLIB_H\n#define MYLIB_H\ntypedef int myint;\nint add(int a, int b);\n#endif\n";
        let parsed = parse_c_header(content);
        assert_eq!(parsed.typedefs.len(), 1);
        assert_eq!(parsed.typedefs[0].name, "myint");
        assert_eq!(parsed.functions.len(), 1);
        assert_eq!(parsed.functions[0].name, "add");
    }

    #[test]
    fn test_generate_bindings_basic() {
        let dir = tempdir().unwrap();
        let header = dir.path().join("mylib.h");
        std::fs::write(&header, "int add(int a, int b);\nvoid noop(void);\n").unwrap();

        let opts = BindingOptions::default();
        let binding = generate_bindings(&header, &opts).unwrap();

        assert!(binding.code.contains("extern \"C\""));
        assert!(binding.code.contains("pub fn add("));
        assert!(binding.code.contains("pub fn noop()"));
        assert!(binding.warnings.is_empty());
    }

    #[test]
    fn test_generate_bindings_missing_file() {
        let opts = BindingOptions::default();
        let result = generate_bindings(Path::new("/nonexistent/header.h"), &opts);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_generate_bindings_rejects_too_many_lines() {
        let dir = tempdir().unwrap();
        let header = dir.path().join("long.h");
        let mut body = String::with_capacity(MAX_HEADER_LINES * 4 + 16);
        for _ in 0..=MAX_HEADER_LINES {
            body.push_str("//x\n");
        }
        std::fs::write(&header, body).unwrap();
        let opts = BindingOptions::default();
        let err = generate_bindings(&header, &opts).unwrap_err();
        assert!(err.contains("too many lines"), "got: {err}");
    }

    #[test]
    fn test_generate_bindings_rejects_oversized_file() {
        let dir = tempdir().unwrap();
        let header = dir.path().join("huge.h");
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&header)
            .unwrap();
        f.set_len(MAX_HEADER_BYTES + 1).unwrap();
        drop(f);
        let opts = BindingOptions::default();
        let err = generate_bindings(&header, &opts).unwrap_err();
        assert!(err.contains("too large"), "got: {err}");
    }

    #[test]
    fn test_generate_bindings_allowlist_functions() {
        let dir = tempdir().unwrap();
        let header = dir.path().join("mylib.h");
        std::fs::write(&header, "int add(int a, int b);\nint sub(int a, int b);\n").unwrap();

        let opts = BindingOptions {
            allowlist_functions: vec!["add".to_string()],
            ..Default::default()
        };
        let binding = generate_bindings(&header, &opts).unwrap();
        assert!(binding.code.contains("pub fn add("));
        assert!(!binding.code.contains("pub fn sub("));
        assert_eq!(binding.warnings.len(), 1);
        assert!(binding.warnings[0].contains("sub"));
    }

    #[test]
    fn test_generate_bindings_mathlib_header() {
        // Verify against the real mathlib.h in the repo
        let header = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/c-ffi/mathlib.h");
        if !header.exists() {
            return;
        }

        let opts = BindingOptions::default();
        let binding = generate_bindings(&header, &opts).unwrap();

        assert!(binding.code.contains("pub fn add("));
        assert!(binding.code.contains("pub fn subtract("));
        assert!(binding.code.contains("pub fn multiply("));
        assert!(binding.code.contains("pub fn fibonacci("));
        assert!(binding.code.contains("pub fn string_length("));
    }

    #[test]
    fn test_generate_bindings_with_typedef() {
        let dir = tempdir().unwrap();
        let header = dir.path().join("types.h");
        std::fs::write(&header, "typedef int handle_t;\nhandle_t open(void);\n").unwrap();

        let opts = BindingOptions::default();
        let binding = generate_bindings(&header, &opts).unwrap();
        assert!(binding.code.contains("pub type handle_t = c_int;"));
        assert!(binding.code.contains("pub fn open()"));
    }
}
