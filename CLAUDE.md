# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build library
cargo build
cargo build --release

# Build eq CLI
cargo build --bin eq --features cli --release

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

# Polyglot demo (TUI works everywhere; GUI needs GPU)
cd examples/polyglot-gui
cargo build --bin polyglot-tui
cargo build --bin polyglot-gui
```

## Architecture

Equilibrium is a Rust library that auto-generates C FFI bindings for foreign-language source files. It implements a three-stage pipeline:

1. **Language Detection** (`src/detector.rs`) — Maps file extensions to one of 10 supported languages (V, Zig, C, C++, C#, Rust, D, Nim, Odin, Hare). `find_compiler()` uses `which` to locate installed compiler binaries. Each `Language` variant knows its extensions, primary/fallback compiler commands, and the CLI flags needed to emit C-compatible output.

2. **Compilation to C** (`src/compiler.rs`) — Invokes the detected compiler with the appropriate flags to produce a `.c` or preprocessed intermediate file plus an optional `.h` header. `compile_to_c()` auto-detects the language; `compile_batch()` handles multiple files; `generate_header()` produces headers for languages with cbindgen/native support (Rust, V).

3. **Binding Generation** (`src/bindings.rs`) — Parses a C header (functions, typedefs, structs) and emits Rust `extern "C"` declarations. `BindingOptions` controls the module name, include paths, symbol allowlists, and `#[derive]` attributes. `c_type_to_rust()` handles the type mapping (e.g. `int` → `c_int`, `char*` → `*mut c_char`).

`src/lib.rs` re-exports the public surface: `detect_language`, `compile_to_c`, `generate_bindings`, `find_compiler`.

### `eq` CLI (`src/bin/eq.rs`)

The `eq` binary (feature-gated behind `cli`) provides four subcommands:

- `eq check` — detects all supported compilers and shows versions/paths
- `eq install [names…]` — installs missing compilers via the best available package manager; multiple compilers install in parallel. Install order: **wax → brew/linuxbrew → apt/dnf/pacman** on Linux/macOS, **wax → winget → scoop** on Windows.
- `eq build [args…]` — runs `cargo build` with all known compiler bin dirs prepended to PATH (linuxbrew, homebrew, `/usr/local/sbin`, etc.)
- `eq generate <header> [-o file]` — emits Rust `extern "C"` bindings from a C header via `equilibrium::generate_bindings`

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
- `examples/polyglot-gui/` — interactive polyglot dashboard with a ratatui TUI (`polyglot-tui`) and a GPUI GUI (`polyglot-gui`). Calls live FFI into C, C++, Zig, Nim, V, D, Odin, and Rust. `build.rs` uses `find_bin()` with hardcoded linuxbrew fallbacks so compilers are found regardless of the shell PATH that cargo inherits.

### Zig FFI notes

Zig objects must be compiled with `-fPIC -OReleaseFast` to link cleanly into Rust's PIE binary. `ReleaseFast` removes safety checks that otherwise pull in Zig's stdlib panic infrastructure, which conflicts with the linker. See `examples/polyglot-gui/build.rs`.

### V FFI on Linux

V's runtime cannot link directly into Rust's PIE binary. The polyglot-gui uses a C shim (`v_module_shim.c`) that implements the same exported symbols with identical semantics.

### Windows build notes

The `polyglot-gui` binary targets D3D11 (no Vulkan needed). Build from Windows PowerShell:
```powershell
cargo build --release --bin polyglot-gui
```
The TUI binary works cross-platform. The `eq` CLI on Windows uses `%TEMP%` as the working directory when invoking winget/scoop to avoid UNC path errors from WSL2 filesystem paths.
