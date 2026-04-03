// Zig module — arithmetic exported as C ABI functions

export fn zig_square(n: i32) i32 {
    return n * n;
}

export fn zig_sum_1_to_n(n: i64) i64 {
    return @divExact(n * (n + 1), 2);
}

export fn zig_is_power_of_two(n: u64) bool {
    return n > 0 and (n & (n - 1)) == 0;
}
