#include <math.h>

int calc_add(int a, int b) {
    return a + b;
}

int calc_subtract(int a, int b) {
    return a - b;
}

int calc_multiply(int a, int b) {
    return a * b;
}

double calc_divide(double a, double b) {
    if (b == 0.0) return 0.0;
    return a / b;
}

int calc_power(int base, int exp) {
    if (exp == 0) return 1;
    int result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
    }
    return result;
}

double calc_sqrt(double n) {
    return sqrt(n);
}
