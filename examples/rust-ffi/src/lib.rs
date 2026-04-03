//! Example Rust library using equilibrium for FFI exports

use equilibrium_rust::{ffi, ffi_struct};

/// Add two integers
#[ffi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two integers
#[ffi]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Calculate factorial
#[ffi]
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        n => (2..=n as u64).product(),
    }
}

/// A 2D point structure
#[ffi_struct]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Calculate distance between two points
#[ffi]
pub fn distance(p1: *const Point, p2: *const Point) -> f64 {
    unsafe {
        let p1 = &*p1;
        let p2 = &*p2;
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
}
