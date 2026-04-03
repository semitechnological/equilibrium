# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test test_detect_v          # Run a single test by name

# Lint / Format — always run cargo fmt before committing
cargo fmt                         # Auto-fix formatting
cargo fmt -- --check              # Check formatting (CI gate)
cargo clippy -- -D warnings       # Lint (warnings as errors)

# Docs
cargo doc --all-features --no-deps

# Run example
cargo run --example using_equilibrium
```

## Architecture

Equilibrium is a Rust library that auto-generates C FFI bindings for foreign-language source files. It implements a three-stage pipeline:

1. **Language Detection** (`src/detector.rs`) — Maps file extensions to one of 10 supported languages (V, Zig, C, C++, C#, Rust, D, Nim, Odin, Hare). `find_compiler()` uses `which` to locate installed compiler binaries. Each `Language` variant knows its extensions, primary/fallback compiler commands, and the CLI flags needed to emit C-compatible output.

2. **Compilation to C** (`src/compiler.rs`) — Invokes the detected compiler with the appropriate flags to produce a `.c` or preprocessed intermediate file plus an optional `.h` header. `compile_to_c()` auto-detects the language; `compile_batch()` handles multiple files; `generate_header()` produces headers for languages with cbindgen/native support (Rust, V).

3. **Binding Generation** (`src/bindings.rs`) — Parses a C header (functions, typedefs, structs) and emits Rust `extern "C"` declarations. `BindingOptions` controls the module name, include paths, symbol allowlists, and `#[derive]` attributes. `c_type_to_rust()` handles the type mapping (e.g. `int` → `c_int`, `char*` → `*mut c_char`).

`src/lib.rs` re-exports the public surface: `detect_language`, `compile_to_c`, `generate_bindings`, `find_compiler`.

### Helper Libraries

Language-specific ergonomic crates live in sibling directories:
- `equilibrium-rust/` — proc macro `#[ffi]` attribute
- `equilibrium-nim/` — Nim type conversion helpers
- `equilibrium-d/` — D `@ffi` UDA and `extern(C)` helpers
- `equilibrium-zig/` — Zig comptime FFI helpers

### Examples

- `examples/using_equilibrium.rs` — demonstrates all three pipeline stages (detect, compile, generate bindings, scan_directory)
- `examples/demo-app/` — minimal end-to-end demo: `build.rs` compiles `math.c` via `cc` and generates bindings with equilibrium; `main.rs` calls C functions through the generated `include!()`d bindings
- `examples/full-demo/` — full demo calling a C calculator library from Rust
- `examples/polyglot-gui/` — polyglot dashboard app with one foreign-language module per supported language (C, C++, Zig, V, D, Nim, Odin, Hare, C#, Rust); `build.rs` compiles C/C++/Zig at build time and sets `cfg(has_c/has_cpp/has_zig)`; the app calls linked FFI functions and renders an HTML dashboard via [crepuscularity-web](https://github.com/semitechnological/crepuscularity)

### Zig FFI notes

Zig objects must be compiled with `-fPIC -OReleaseFast` to link cleanly into Rust's PIE binary. `ReleaseFast` removes safety checks that otherwise pull in Zig's stdlib panic infrastructure, which conflicts with the linker. See `examples/polyglot-gui/build.rs`.
