# Rust FFI Example

This example demonstrates using equilibrium-rust to export Rust functions via FFI.

## Structure

```
mathlib/           # Rust library with FFI exports
├── src/lib.rs     # Functions marked with #[ffi]
├── Cargo.toml     # Depends on equilibrium-rust
└── build/         # Compiled output
```

## Building

```bash
cargo build --release
```

This produces `target/release/libmathlib.so` (or `.dylib` on macOS, `.dll` on Windows).

## Using with equilibrium-ffi

```bash
# From a generated C header, e.g.:
# eq generate path/to/mathlib.h -o bindings.rs

# This example crate builds standalone:
cargo build
```

## What it demonstrates

1. **Simple FFI functions** - `add`, `multiply` use `#[ffi]` macro
2. **Complex logic** - `factorial` with pattern matching
3. **Structs** - `Point` with `#[ffi_struct]` for C layout
4. **Pointers** - `distance` takes pointer arguments

## Without equilibrium-rust

You'd need to write:
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

#[repr(C)]
pub struct Point { pub x: f64, pub y: f64 }
```

## With equilibrium-rust

Much cleaner:
```rust
#[ffi]
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[ffi_struct]
pub struct Point { pub x: f64, pub y: f64 }
```
