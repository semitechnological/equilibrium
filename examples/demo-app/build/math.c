# 1 "/home/undivisible/equilibrium/demo-app/foreign-code/math.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 406 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/undivisible/equilibrium/demo-app/foreign-code/math.c" 2

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
