//! Example showing the full equilibrium-ffi pipeline:
//! detect language → compile to C → generate Rust FFI bindings

use equilibrium_ffi::{
    compile_to_c, detect_language, find_compiler, generate_bindings, BindingOptions,
};
use std::path::Path;

fn main() {
    println!("=== Equilibrium Usage Example ===\n");

    // ── 1. Language detection ────────────────────────────────────────────────
    println!("1. Language Detection:");
    let files = ["examples/c-ffi/mathlib.c", "examples/rust-ffi/src/lib.rs"];

    for file in files {
        let path = Path::new(file);
        if path.exists() {
            if let Some(lang) = detect_language(path) {
                println!("   {} → {:?}", file, lang);
            }
        }
    }

    // ── 2. Compiler discovery ────────────────────────────────────────────────
    println!("\n2. Available Compilers:");
    for lang in equilibrium_ffi::Language::all() {
        if let Some(info) = find_compiler(*lang) {
            let compiler = info.compiler.unwrap_or_default();
            let version = info
                .version
                .map(|v| format!(" ({})", v))
                .unwrap_or_default();
            println!("   {:?}: {}{}", lang, compiler, version);
        }
    }

    // ── 3. Compile C source to preprocessed output ───────────────────────────
    println!("\n3. Compiling C Library:");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let c_source = Path::new(manifest_dir).join("examples/c-ffi/mathlib.c");

    if !c_source.exists() {
        println!("   ✗ Source file not found: {}", c_source.display());
        return;
    }

    let output_dir = Path::new(manifest_dir).join("examples/c-ffi/build");
    std::fs::create_dir_all(&output_dir).ok();

    let _compile_result = match compile_to_c(&c_source, &output_dir) {
        Ok(r) => {
            println!("   ✓ Compiled successfully");
            println!("   Output: {}", r.output_path.display());
            r
        }
        Err(e) => {
            println!("   ✗ Compilation failed: {}", e);
            return;
        }
    };

    // ── 4. Generate Rust FFI bindings from the header ────────────────────────
    println!("\n4. Generating Rust FFI Bindings:");

    // Use the hand-written header that ships with the example.
    // For languages that emit headers automatically (e.g. V with -backend c),
    // you would use compile_result.header_path directly.
    let header = Path::new(manifest_dir).join("examples/c-ffi/mathlib.h");

    let opts = BindingOptions::default();
    match generate_bindings(&header, &opts) {
        Ok(binding) => {
            println!("   ✓ Generated bindings from {}", header.display());
            if !binding.warnings.is_empty() {
                for w in &binding.warnings {
                    println!("   ⚠ {}", w);
                }
            }

            // Write the bindings to a file so callers can `include!` them
            let bindings_out = output_dir.join("mathlib_bindings.rs");
            if std::fs::write(&bindings_out, &binding.code).is_ok() {
                println!("   Written: {}", bindings_out.display());
            }

            // Print a snippet of the generated code
            println!("\n   Generated bindings (excerpt):");
            for line in binding
                .code
                .lines()
                .filter(|l| !l.starts_with("//!"))
                .take(20)
            {
                println!("   {}", line);
            }
        }
        Err(e) => {
            println!("   ✗ Binding generation failed: {}", e);
            return;
        }
    }

    // ── 5. Directory scanning ────────────────────────────────────────────────
    println!("\n5. Scanning for Source Files:");
    let scan_root = Path::new(manifest_dir).join("examples");
    let found = equilibrium_ffi::scan_directory(&scan_root);
    for (path, lang) in &found {
        let rel = path.strip_prefix(manifest_dir).unwrap_or(path);
        println!("   {:?}  {}", lang, rel.display());
    }
    println!("   Total: {} source file(s) found", found.len());

    println!("\n=== Example Complete ===");
    println!("\nThe generated bindings file can be used in Rust like:");
    println!("   include!(concat!(env!(\"OUT_DIR\"), \"/mathlib_bindings.rs\"));");
    println!("   let result = unsafe {{ add(3, 4) }};");
    println!("   assert_eq!(result, 7);");
}
