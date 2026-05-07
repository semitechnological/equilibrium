// D module — numeric algorithms
// Compiled with: ldc2 -c -of d_module.o -HC d_module.d

import core.stdc.stdlib;

extern(C):

int d_abs(int n) {
    return n < 0 ? -n : n;
}

long d_triangular(int n) {
    return cast(long)n * (n + 1) / 2;
}

int d_clamp(int n, int lo, int hi) {
    if (n < lo) return lo;
    if (n > hi) return hi;
    return n;
}

int d_collatz_steps(int n) {
    if (n <= 0) return 0;
    int steps = 0;
    long x = n;
    while (x != 1) {
        x = (x % 2 == 0) ? x / 2 : x * 3 + 1;
        steps++;
    }
    return steps;
}
