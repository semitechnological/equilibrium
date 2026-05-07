// C++ module — string synthesis and dense number theory
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

long long cpp_primorial(int n) {
    auto is_prime = [](int x) {
        if (x < 2) return false;
        if (x == 2) return true;
        if (x % 2 == 0) return false;
        for (int i = 3; i * i <= x; i += 2) {
            if (x % i == 0) return false;
        }
        return true;
    };

    long long result = 1;
    for (int i = 2; i <= n; ++i) {
        if (is_prime(i)) result *= i;
    }
    return result;
}

int cpp_digit_sum(int n) {
    n = n < 0 ? -n : n;
    int sum = 0;
    while (n > 0) {
        sum += n % 10;
        n /= 10;
    }
    return sum;
}

} // extern "C"
