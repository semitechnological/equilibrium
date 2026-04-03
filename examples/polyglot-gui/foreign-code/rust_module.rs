// Rust module — number theory helpers
// This module is compiled as part of the main binary (not as a separate FFI target)
// because Rust is the host language. The functions below are exported as C ABI
// from a hypothetical cdylib — shown here as source documentation.

#[no_mangle]
pub extern "C" fn rust_is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_next_prime(n: u64) -> u64 {
    let mut candidate = n + 1;
    loop {
        if rust_is_prime(candidate) {
            return candidate;
        }
        candidate += 1;
    }
}
