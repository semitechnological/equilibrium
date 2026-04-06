# Equilibrium

**Automatic C FFI generation for C-compiling languages**

Equilibrium auto-detects source files in various programming languages, compiles them to C intermediate representation, and generates Rust bindings so you can call foreign code like native modules.

## `eq` CLI

The `eq` CLI manages compilers and builds polyglot projects.

```bash
# Build
cargo install --path . --features cli

# Check which compilers are installed
eq check

# Install missing compilers (interactive multi-select, parallel)
eq install

# Install specific compilers
eq install zig nim d odin

# Build a project with all compilers on PATH
eq build --release --bin my-app

# Generate Rust FFI bindings from a C header
eq generate mylib.h -o src/mylib_ffi.rs
```

**Install order per platform:**
- **Linux**: wax → brew/linuxbrew → apt / dnf / pacman
- **macOS**: wax → brew
- **Windows**: winget → scoop

Multiple compilers install in parallel.

## How It Works

### 1. Language Detection

```rust
use equilibrium::detect_language;
use std::path::Path;

let source = Path::new("mylib.v");
if let Some(lang) = detect_language(source) {
    println!("Detected: {:?}", lang); // Language::V
}
```

### 2. Compiler Detection

```rust
use equilibrium::{find_compiler, Language};

if let Some(info) = find_compiler(Language::Zig) {
    println!("Found: {}", info.compiler.unwrap());
}
```

### 3. Compilation to C

```rust
use equilibrium::compile_to_c;
use std::path::Path;

let source = Path::new("math.v");
let output_dir = Path::new("./build");

match compile_to_c(source, output_dir) {
    Ok(result) => println!("Output: {:?}", result.output_path),
    Err(e) => eprintln!("Compilation failed: {}", e),
}
```

### 4. Binding Generation

```rust
use equilibrium::generate_bindings;

let bindings = generate_bindings(&c_header_path, &Default::default())?;
println!("{}", bindings.code);
```

## Supported Languages

| Language | Compiler | Notes |
|----------|----------|-------|
| **V (Vlang)** | `v` | `-backend c` outputs C |
| **Zig** | `zig` | `build-obj -OReleaseFast -fPIC` |
| **C** | `clang`/`gcc` | Already C (preprocessed) |
| **C++** | `clang++`/`g++` | Compiled to object files |
| **C#** | `dotnet` | Native AOT |
| **Rust** | `rustc` | cbindgen for header generation |
| **D** | `ldc2`/`dmd`/`gdc` | `-HC` flag for C headers |
| **Nim** | `nim` | Compiles to C by default, `--mm:none --app:staticlib` |
| **Odin** | `odin` | `-build-mode:obj -reloc-mode:pic` |
| **Hare** | `hare` | QBE backend (Linux only) |

## Installation

```toml
[dependencies]
equilibrium = { git = "https://github.com/semitechnological/equilibrium" }
```

For the `eq` CLI:
```toml
[dependencies]
equilibrium = { git = "https://github.com/semitechnological/equilibrium", features = ["cli"] }
```

Or install globally:
```bash
cargo install --git https://github.com/semitechnological/equilibrium --features cli
```

## Architecture

```
┌─────────────────┐
│  Source Files   │
│  (.v, .zig, .d) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Detector      │ ◄─── Auto-detect language + compiler
│  detector.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Compiler      │ ◄─── Invoke with language-specific flags
│  compiler.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  C Output       │
│  (.c, .h, .o)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Bindings      │ ◄─── Parse C headers → Rust FFI
│  bindings.rs    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Rust Code      │
│  (ready to use) │
└─────────────────┘
```

## Helper Libraries

| Language | Library | Description |
|----------|---------|-------------|
| **Rust** | `equilibrium-rust` | `#[ffi]` proc macro for automatic `extern "C"` |
| **Nim** | `equilibrium.nim` | Type conversion helpers and export utilities |
| **D** | `equilibrium.d` | `@ffi` UDA and `extern(C)` helpers |
| **Zig** | `equilibrium.zig` | Comptime FFI helpers and type conversions |

## Polyglot Demo

`examples/polyglot-gui/` is a live demo calling all 8 languages from one binary.

```bash
cd examples/polyglot-gui

# TUI (works everywhere including WSL2)
cargo build --bin polyglot-tui
./target/debug/polyglot-tui

# GUI (requires GPU: D3D11 on Windows, Metal on macOS, Vulkan on Linux)
cargo build --bin polyglot-gui
./target/debug/polyglot-gui
```

Or use `eq build` to ensure all compilers are on PATH:
```bash
cd examples/polyglot-gui
eq build --bin polyglot-tui
```

## Testing

```bash
cargo test
```

## CI/CD

- `.github/workflows/ci.yml` — tests on Linux/macOS/Windows
- `.github/actions/setup-equilibrium/` — reusable action for your projects

```yaml
- uses: semitechnological/equilibrium/.github/actions/setup-equilibrium@main
  with:
    install-zig: true
    install-nim: true
```

## License

MIT
