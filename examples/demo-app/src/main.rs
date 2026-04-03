//! Polyglot Calculator - Demonstrates equilibrium calling C functions from Rust

use std::path::{Path, PathBuf};

fn main() {
    println!("=== Polyglot Calculator Demo ===\n");
    
    // Get absolute paths
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let c_source = manifest_dir.join("foreign-code/math.c");
    let build_dir = manifest_dir.join("build");
    
    // Step 1: Compile C code
    println!("Step 1: Compiling C math library...");
    println!("  Source: {}", c_source.display());
    
    std::fs::create_dir_all(&build_dir).unwrap();
    
    match equilibrium::compile_to_c(&c_source, &build_dir) {
        Ok(result) => {
            println!("✓ Compiled successfully");
            println!("  Output: {}", result.output_path.display());
            
            // Step 2: Parse the output to find function signatures
            println!("\nStep 2: Analyzing functions...");
            if let Ok(content) = std::fs::read_to_string(&result.output_path) {
                let functions: Vec<&str> = content.lines()
                    .filter(|l| l.contains("int c_"))
                    .filter(|l| l.contains('('))
                    .filter(|l| !l.trim().starts_with("//"))
                    .filter(|l| !l.trim().starts_with('#'))
                    .take(5)
                    .collect();
                
                if !functions.is_empty() {
                    println!("  Found {} function(s):", functions.len());
                    for func in functions {
                        let trimmed = func.trim();
                        if trimmed.len() < 80 {
                            println!("    {}", trimmed);
                        }
                    }
                } else {
                    println!("  (Functions are in preprocessed output)");
                }
            }
            
            println!("\n=== Next Steps ===");
            println!("\nTo actually use these C functions in Rust:");
            println!("\n1. Generate FFI bindings:");
            println!("   let bindings = equilibrium::generate_bindings(&header)?;");
            println!("\n2. Write bindings to file:");
            println!("   std::fs::write(\"src/ffi.rs\", bindings.rust_code)?;");
            println!("\n3. Use in Rust:");
            println!("   mod ffi;");
            println!("   let sum = unsafe {{ ffi::c_add(5, 3) }};");
            println!("   println!(\"5 + 3 = {{}}\", sum);");
            
            println!("\n✓ Demo complete!");
        }
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            eprintln!("\nThis might mean:");
            eprintln!("  - C compiler not found (install clang or gcc)");
            eprintln!("  - Source file missing");
            std::process::exit(1);
        }
    }
}
