// C++ module — string utilities and combinatorics
#include "cpp_module.h"
#include <cstring>

extern "C" {

int cpp_strlen(const char *s) {
    return static_cast<int>(std::strlen(s));
}

long long cpp_factorial(int n) {
    if (n <= 1) return 1LL;
    long long result = 1;
    for (int i = 2; i <= n; ++i) result *= i;
    return result;
}

int cpp_is_prime(int n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

} // extern "C"
