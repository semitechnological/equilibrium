// Simple C math library
int c_add(int a, int b) {
    return a + b;
}

int c_factorial(int n) {
    if (n <= 1) return 1;
    int result = 1;
    for (int i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}
