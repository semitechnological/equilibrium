/* C module — basic integer math */
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
