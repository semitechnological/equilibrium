//! Example showing how to use equilibrium to detect and compile foreign code

use equilibrium::{compile_to_c, detect_language, find_compiler};
use std::path::Path;

fn main() {
    println!("=== Equilibrium Usage Example ===\n");

    // Example 1: Detect language
    println!("1. Language Detection:");
    let files = [
        "examples/c-ffi/mathlib.c",
        "examples/rust-ffi/src/lib.rs",
    ];

    for file in files {
        let path = Path::new(file);
        if path.exists() {
            if let Some(lang) = detect_language(path) {
                println!("   {} -> {:?}", file, lang);
            }
        }
    }

    // Example 2: Find compilers
    println!("\n2. Available Compilers:");
    for lang in equilibrium::Language::all() {
        if let Some(info) = find_compiler(*lang) {
            let compiler = info.compiler.unwrap_or_default();
            println!("   {:?}: {}", lang, compiler);
        }
    }

    // Example 3: Compile C code
    println!("\n3. Compiling C Library:");
    
    // Get the workspace root
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let c_source = Path::new(manifest_dir).join("examples/c-ffi/mathlib.c");
    
    if c_source.exists() {
        let output_dir = Path::new(manifest_dir).join("examples/c-ffi/build");
        std::fs::create_dir_all(&output_dir).ok();

        match compile_to_c(&c_source, &output_dir) {
            Ok(result) => {
                println!("   ✓ Compiled successfully");
                println!("   Output: {}", result.output_path.display());
                if let Some(header) = result.header_path {
                    println!("   Header: {}", header.display());
                }
                
                // Show a preview of the preprocessed output
                if let Ok(content) = std::fs::read_to_string(&result.output_path) {
                    let lines: Vec<&str> = content.lines().collect();
                    let relevant: Vec<&str> = lines.iter()
                        .filter(|l| !l.starts_with('#'))
                        .filter(|l| !l.trim().is_empty())
                        .copied()
                        .take(10)
                        .collect();
                    
                    if !relevant.is_empty() {
                        println!("\n   Preview (non-preprocessor lines):");
                        for line in relevant {
                            println!("     {}", line);
                        }
                    }
                }
            }
            Err(e) => {
                println!("   ✗ Compilation failed: {}", e);
            }
        }
    } else {
        println!("   ✗ Source file not found: {}", c_source.display());
    }

    println!("\n=== Example Complete ===");
}
