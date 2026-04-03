#include "mathlib.h"
#include <string.h>

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}

int multiply(int a, int b) {
    return a * b;
}

int string_length(const char* str) {
    if (!str) return 0;
    return strlen(str);
}

void reverse_string(char* str) {
    if (!str) return;
    int len = strlen(str);
    for (int i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
}

unsigned long fibonacci(unsigned int n) {
    if (n <= 1) return n;
    unsigned long a = 0, b = 1;
    for (unsigned int i = 2; i <= n; i++) {
        unsigned long temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}
