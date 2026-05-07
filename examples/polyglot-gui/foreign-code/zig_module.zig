// Zig module — arithmetic exported as C ABI functions

export fn zig_square(n: i64) i64 {
    return n * n;
}

export fn zig_sum_1_to_n(n: i64) i64 {
    return @divExact(n * (n + 1), 2);
}

export fn zig_is_power_of_two(n: u64) bool {
    return n > 0 and (n & (n - 1)) == 0;
}

export fn zig_spiral_sum(n: i64) i64 {
    var acc: i64 = 0;
    var i: i64 = 1;
    while (i <= n) : (i += 1) {
        acc += i * i + i;
    }
    return acc;
}

export fn zig_chaos_fold(n: i64) i64 {
    var x = n;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    return x;
}
