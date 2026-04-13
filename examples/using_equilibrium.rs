//! Smallest possible `load()` demo.

fn main() {
    for src in [
        "examples/c-ffi/mathlib.c",
        "examples/polyglot-gui/foreign-code/zig_module.zig",
        "examples/polyglot-gui/foreign-code/nim_module.nim",
    ] {
        match equilibrium_ffi::load(src) {
            Ok(lib) => println!("{} -> {}", src, lib.output_path.display()),
            Err(e) => println!("{} -> {}", src, e),
        }
    }
}
