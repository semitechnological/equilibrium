// Hare module — safe integer operations
// Compiled with: hare build -o hare_module.o hare_module.ha

use fmt;

export fn hare_sign(n: i32) i32 = {
    if (n > 0) return 1;
    if (n < 0) return -1;
    return 0;
};

export fn hare_div_safe(a: i32, b: i32) i32 = {
    if (b == 0) return 0;
    return a / b;
};
