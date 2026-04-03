# Equilibrium

**Automatic C FFI generation for C-compiling languages**

Equilibrium auto-detects source files in various programming languages, compiles them to C intermediate representation, and generates Rust bindings so you can call foreign code like native modules.

## How It Works

### 1. Language Detection

Equilibrium detects languages by file extension:

```rust
use equilibrium::detect_language;
use std::path::Path;

let source = Path::new("mylib.v");
if let Some(lang) = detect_language(source) {
    println!("Detected: {:?}", lang); // Language::V
}
```

### 2. Compiler Detection

It automatically finds installed compilers on your system:

```rust
use equilibrium::{find_compiler, Language};

if let Some(info) = find_compiler(Language::Zig) {
    println!("Found: {}", info.compiler.unwrap());
    println!("Version: {}", info.version.unwrap());
}
```

### 3. Compilation to C

Equilibrium invokes the appropriate compiler with language-specific flags to generate C output:

```rust
use equilibrium::compile_to_c;
use std::path::Path;

let source = Path::new("math.v");
let output_dir = Path::new("./build");

match compile_to_c(source, output_dir) {
    Ok(result) => {
        println!("Output: {:?}", result.output_path);
        println!("Header: {:?}", result.header_path);
    }
    Err(e) => eprintln!("Compilation failed: {}", e),
}
```

### 4. Binding Generation

Generate Rust FFI bindings from the C output:

```rust
use equilibrium::generate_bindings;

let bindings = generate_bindings(&c_header_path)?;
println!("{}", bindings.rust_code);
```

## Supported Languages

| Language | Compiler | C Backend Method |
|----------|----------|------------------|
| **V (Vlang)** | `v` | `-backend c` flag outputs C code |
| **Zig** | `zig` | `build-obj` + export declarations |
| **C** | `clang`/`gcc` | Already C (preprocessed) |
| **C++** | `clang++`/`g++` | Compiled to object files |
| **C#** | `csc`/`mono` | Native AOT compilation |
| **Rust** | `rustc` | `cbindgen` for header generation |
| **D** | `ldc2`/`dmd`/`gdc` | `-HC` flag for C headers |
| **Nim** | `nim` | Compiles to C by default |
| **Odin** | `odin` | Object file generation |
| **Hare** | `hare` | QBE backend produces objects |

## Compilation Examples

### V Language
```bash
v -o output.c -backend c mylib.v
```

### Zig
```bash
zig build-obj -femit-bin=output.o mylib.zig
```

### D Language
```bash
ldc2 -c -of=output.o -HC mylib.d  # -HC generates C header
```

### Nim
```bash
nim c --nimcache:. -o:output.o mylib.nim
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
equilibrium = { git = "https://github.com/semitechnological/equilibrium" }
```

## Architecture

```
┌─────────────────┐
│  Source Files   │
│  (.v, .zig, .d) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Detector      │ ◄─── Auto-detect language + compiler
│  detector.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Compiler      │ ◄─── Invoke with language-specific flags
│  compiler.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  C Output       │
│  (.c, .h, .o)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Bindings      │ ◄─── Parse C headers → Rust FFI
│  bindings.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Rust Code      │
│  (ready to use) │
└─────────────────┘
```

## Testing

```bash
cargo test
```

All 14 tests verify:
- Language detection by extension
- Compiler availability checking
- C type to Rust type conversion
- Function signature parsing
- Error handling

## Future Plans

- [ ] Watch mode for auto-recompilation
- [ ] Direct integration with build.rs
- [ ] Parallel batch compilation
- [ ] Automatic dependency detection
- [ ] Cross-compilation support

## License

MIT
