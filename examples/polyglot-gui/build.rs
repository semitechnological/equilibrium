use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Declare custom cfg keys so rustc doesn't warn about unknown cfgs
    println!("cargo::rustc-check-cfg=cfg(has_c)");
    println!("cargo::rustc-check-cfg=cfg(has_cpp)");
    println!("cargo::rustc-check-cfg=cfg(has_zig)");

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let foreign = manifest.join("foreign-code");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // ── C module (always available) ─────────────────────────────────────────
    cc::Build::new()
        .file(foreign.join("c_module.c"))
        .compile("c_module");
    println!("cargo:rustc-cfg=has_c");
    println!("cargo:rerun-if-changed=foreign-code/c_module.c");
    println!("cargo:rerun-if-changed=foreign-code/c_module.h");

    // Generate C bindings via equilibrium
    emit_bindings(&foreign.join("c_module.h"), &out_dir, "c_bindings.rs");

    // ── C++ module (always available) ──────────────────────────────────────
    cc::Build::new()
        .cpp(true)
        .file(foreign.join("cpp_module.cpp"))
        .compile("cpp_module");
    println!("cargo:rustc-cfg=has_cpp");
    println!("cargo:rerun-if-changed=foreign-code/cpp_module.cpp");
    println!("cargo:rerun-if-changed=foreign-code/cpp_module.h");

    emit_bindings(&foreign.join("cpp_module.h"), &out_dir, "cpp_bindings.rs");

    // ── Zig module (when zig is on PATH) ───────────────────────────────────
    if which::which("zig").is_ok() {
        let obj = out_dir.join("zig_module.o");
        let status = Command::new("zig")
            .args([
                "build-obj",
                "-fPIC",
                "-OReleaseFast", // No safety checks → no panic/stdlib linkage
                &format!("-femit-bin={}", obj.display()),
                foreign.join("zig_module.zig").to_str().unwrap(),
            ])
            .status();

        if status.map(|s| s.success()).unwrap_or(false) && obj.exists() {
            cc::Build::new().object(&obj).compile("zig_module");
            println!("cargo:rustc-cfg=has_zig");
        }
    }
    println!("cargo:rerun-if-changed=foreign-code/zig_module.zig");
}

fn emit_bindings(header: &std::path::Path, out_dir: &std::path::Path, filename: &str) {
    let opts = equilibrium::BindingOptions::default();
    match equilibrium::generate_bindings(header, &opts) {
        Ok(binding) => {
            let _ = std::fs::write(out_dir.join(filename), &binding.code);
        }
        Err(e) => {
            eprintln!(
                "cargo:warning=equilibrium binding failed for {:?}: {}",
                header, e
            );
        }
    }
}
