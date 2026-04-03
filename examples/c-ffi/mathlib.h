#ifndef MATHLIB_H
#define MATHLIB_H

// Simple arithmetic functions
int add(int a, int b);
int subtract(int a, int b);
int multiply(int a, int b);

// String utilities
int string_length(const char* str);
void reverse_string(char* str);

// Fibonacci
unsigned long fibonacci(unsigned int n);

#endif // MATHLIB_H
