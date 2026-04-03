# C FFI Example

This example shows using equilibrium with a plain C library.

## Structure

```
c-ffi/
├── mathlib.h      # C header file
├── mathlib.c      # C implementation
├── build.sh       # Build script
└── libmathlib.so  # Compiled shared library
```

## Building

```bash
./build.sh
```

This creates `libmathlib.so` (or `.dylib` on macOS).

## Using with equilibrium

```rust
use equilibrium::{compile_to_c, detect_language};
use std::path::Path;

let source = Path::new("mathlib.c");
let output_dir = Path::new("./build");

let result = compile_to_c(source, output_dir)?;
println!("Compiled: {:?}", result.output_path);
```

## What it demonstrates

1. **Plain C** - No special attributes needed, C is already C!
2. **String handling** - C string functions
3. **Algorithms** - Iterative fibonacci
4. **Shared library** - Building with gcc

## Testing

```bash
# Build the library
./build.sh

# Use equilibrium to process it
cd ../..
cargo run --example c-usage
```
