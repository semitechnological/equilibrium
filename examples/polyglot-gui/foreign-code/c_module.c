/* C module — signal processing and integer chaos */
#include "c_module.h"

int c_add(int a, int b) {
    return a + b;
}

int c_gcd(int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

long c_fibonacci(int n) {
    if (n <= 1) return (long)n;
    long a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        long t = a + b;
        a = b;
        b = t;
    }
    return b;
}

long c_wave_hash(int n) {
    long x = n;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    return x;
}

long c_collatz_steps(int n) {
    if (n <= 0) return 0;
    long steps = 0;
    long x = n;
    while (x != 1) {
        x = (x % 2 == 0) ? x / 2 : x * 3 + 1;
        steps++;
    }
    return steps;
}
