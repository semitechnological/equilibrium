use std::path::PathBuf;

fn main() {
    let foreign = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("foreign-code");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let lib = equilibrium_ffi::load(foreign.join("calculator.c")).expect("load calculator.c");
    if let Some(code) = lib.bindings_code() {
        std::fs::write(out_dir.join("calculator_bindings.rs"), code).expect("write bindings");
    }
    println!("cargo:rerun-if-changed=foreign-code/*");
}
