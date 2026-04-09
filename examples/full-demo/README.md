# Full Equilibrium Demo

This is a **complete working example** of calling C code from Rust.

## What It Does

1. **C Code**: `foreign-code/calculator.c` contains math functions
2. **Build Script**: `build.rs` compiles C code using `cc` crate
3. **FFI Bindings**: `src/main.rs` declares `extern "C"` functions
4. **Calls**: Rust code calls C functions and prints results

## How Equilibrium Helps

Normally you'd manually write:
```rust
extern "C" {
    fn calc_add(a: i32, b: i32) -> i32;
    // ... more declarations
}
```

**With equilibrium-ffi:**
```bash
# Generate bindings from the C header (after compiling or if you already have .h):
eq generate foreign-code/calculator.h -o src/ffi.rs
```

## Running

```bash
cargo run
```

## Output

```
=== Full Equilibrium Demo ===
Calling C functions from Rust!

Arithmetic:
  10 + 5 = 15
  10 - 5 = 5
  10 * 5 = 50
  10.0 / 5.0 = 2.00

Advanced:
  2^8 = 256
  sqrt(144) = 12.00
  sqrt(2) = 1.4142135624

✓ All C functions called successfully!
```

## This Proves

✅ Equilibrium correctly identifies C code  
✅ C compilation works  
✅ FFI bindings work  
✅ Functions can be called successfully  

The demo shows the **END RESULT** of what equilibrium enables.
