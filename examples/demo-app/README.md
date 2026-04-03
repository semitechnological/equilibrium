# Polyglot Calculator Demo

A demonstration app showing equilibrium compiling and calling C code from Rust.

## What it does

1. Takes a C source file (`foreign-code/math.c`)
2. Compiles it using equilibrium
3. Analyzes the compiled output
4. Shows how you'd generate bindings and call the functions

## Running

```bash
cargo run
```

## Output

```
=== Polyglot Calculator Demo ===

Step 1: Compiling C math library...
✓ Compiled successfully
  Output: build/math.c

Step 2: Analyzing functions...
  Found functions:
    int c_add(int a, int b) {
    int c_factorial(int n) {

✓ Demo complete!
```

## Next Steps

To actually call these functions:

1. **Generate bindings** (we'd need to generate a proper header)
2. **Link the library** (add to build.rs)
3. **Call from Rust**:
   ```rust
   extern "C" {
       fn c_add(a: i32, b: i32) -> i32;
       fn c_factorial(n: i32) -> i32;
   }
   
   unsafe {
       println!("5 + 3 = {}", c_add(5, 3));
       println!("5! = {}", c_factorial(5));
   }
   ```
