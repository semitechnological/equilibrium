//! Polyglot Calculator — end-to-end equilibrium-ffi demo
//!
//! build.rs compiles foreign-code/math.c and uses equilibrium-ffi to generate
//! Rust FFI bindings automatically. This file just calls those functions.

// Pull in the bindings that build.rs wrote to OUT_DIR
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/math_bindings.rs"));
}

fn main() {
    println!("=== Polyglot Calculator Demo ===");
    println!("C functions called directly from Rust via auto-generated FFI bindings\n");

    unsafe {
        // Basic arithmetic
        let sum = ffi::c_add(7, 3);
        println!("c_add(7, 3)       = {}", sum);
        assert_eq!(sum, 10);

        // Factorial
        let f5 = ffi::c_factorial(5);
        println!("c_factorial(5)    = {}", f5);
        assert_eq!(f5, 120);

        let f10 = ffi::c_factorial(10);
        println!("c_factorial(10)   = {}", f10);
        assert_eq!(f10, 3628800);
    }

    println!("\n✓ All assertions passed!");
    println!("\nHow it works:");
    println!("  1. build.rs compiles foreign-code/math.c into a static library via the cc crate");
    println!("  2. build.rs calls equilibrium_ffi::generate_bindings() on math.h");
    println!("  3. The generated bindings are written to $OUT_DIR/math_bindings.rs");
    println!("  4. This file includes those bindings with include!() and calls them directly");

    // Also demonstrate the equilibrium-ffi library functions at runtime
    println!("\n--- Runtime equilibrium-ffi API ---");
    let manifest = env!("CARGO_MANIFEST_DIR");
    let c_source = std::path::Path::new(manifest).join("foreign-code/math.c");

    if let Some(lang) = equilibrium_ffi::detect_language(&c_source) {
        println!(
            "detect_language({:?}) = {:?}",
            c_source.file_name().unwrap(),
            lang
        );
    }

    if let Some(info) = equilibrium_ffi::find_compiler(equilibrium_ffi::Language::C) {
        println!(
            "find_compiler(C) = {} {}",
            info.compiler.as_deref().unwrap_or("?"),
            info.version.as_deref().unwrap_or("")
        );
    }
}
