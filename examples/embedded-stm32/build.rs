// Frictionless embedded C library bindings with equilibrium-ffi
//
// This build.rs automatically discovers all C headers in the stm32-libs/
// directory and generates Rust FFI bindings for them.

fn main() {
    println!("cargo:rerun-if-changed=stm32-libs/");

    // ZERO-CONFIG AUTO-DISCOVERY
    // Just point equilibrium-ffi at your C library folder and it does the rest!
    match equilibrium_ffi::scan_c_libraries("stm32-libs").generate_all() {
        Ok(result) => {
            println!(
                "✓ Generated bindings for {} libraries in {}",
                result.libraries.len(),
                result.output_dir.display()
            );

            for lib in &result.libraries {
                println!(
                    "  - {} ({} headers)",
                    lib.library.name,
                    lib.library.headers.len()
                );
            }
        }
        Err(e) => {
            // Don't panic in build.rs - let the build continue even if libs aren't present
            eprintln!("Warning: Failed to generate bindings: {}", e);
            eprintln!("This is expected if you haven't added STM32 libraries yet.");
        }
    }
}
