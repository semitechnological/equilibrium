// D module — numeric algorithms
// Compiled with: ldc2 -c -of d_module.o -HC d_module.d

import core.stdc.stdlib;

extern(C):

int d_abs(int n) {
    return n < 0 ? -n : n;
}

int d_clamp(int n, int lo, int hi) {
    if (n < lo) return lo;
    if (n > hi) return hi;
    return n;
}

long d_triangular(int n) {
    return cast(long)n * (n + 1) / 2;
}
