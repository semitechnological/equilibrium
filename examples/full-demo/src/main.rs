//! Full Demo - Actually calling C functions from Rust via FFI

extern "C" {
    fn calc_add(a: i32, b: i32) -> i32;
    fn calc_subtract(a: i32, b: i32) -> i32;
    fn calc_multiply(a: i32, b: i32) -> i32;
    fn calc_divide(a: f64, b: f64) -> f64;
    fn calc_power(base: i32, exp: i32) -> i32;
    fn calc_sqrt(n: f64) -> f64;
}

fn main() {
    println!("=== Full Equilibrium Demo ===");
    println!("Calling C functions from Rust!\n");
    
    unsafe {
        // Arithmetic operations
        println!("Arithmetic:");
        println!("  10 + 5 = {}", calc_add(10, 5));
        println!("  10 - 5 = {}", calc_subtract(10, 5));
        println!("  10 * 5 = {}", calc_multiply(10, 5));
        println!("  10.0 / 5.0 = {:.2}", calc_divide(10.0, 5.0));
        
        // Advanced operations
        println!("\nAdvanced:");
        println!("  2^8 = {}", calc_power(2, 8));
        println!("  sqrt(144) = {:.2}", calc_sqrt(144.0));
        println!("  sqrt(2) = {:.10}", calc_sqrt(2.0));
    }
    
    println!("\n✓ All C functions called successfully!");
    println!("\nThis demonstrates:");
    println!("  - C code compiled via cc crate");
    println!("  - Rust FFI declarations");
    println!("  - Calling C functions from Rust");
    println!("  - Equilibrium would automate the FFI declaration part");
}
