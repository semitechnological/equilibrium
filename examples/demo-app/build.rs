use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let foreign = manifest.join("foreign-code");

    // Step 1: Compile the C library and link it
    cc::Build::new()
        .file(foreign.join("math.c"))
        .compile("math");

    println!("cargo:rerun-if-changed=foreign-code/math.c");
    println!("cargo:rerun-if-changed=foreign-code/math.h");

    // Step 2: Generate Rust FFI bindings from the header using equilibrium-ffi
    let header = foreign.join("math.h");
    let opts = equilibrium_ffi::BindingOptions::default();

    match equilibrium_ffi::generate_bindings(&header, &opts) {
        Ok(binding) => {
            let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
            let bindings_path = out_dir.join("math_bindings.rs");
            std::fs::write(&bindings_path, &binding.code)
                .expect("failed to write generated bindings");
        }
        Err(e) => {
            panic!("equilibrium-ffi failed to generate bindings: {}", e);
        }
    }
}
