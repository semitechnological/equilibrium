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

# Lint / Format
cargo fmt -- --check              # Check formatting
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

- `examples/using_equilibrium.rs` — demonstrates the three pipeline stages (detect, compile, generate bindings)
- `examples/full-demo/` — end-to-end working demo that compiles a C calculator library and calls it from Rust via the `cc` crate
