#ifndef CPP_MODULE_H
#define CPP_MODULE_H

#ifdef __cplusplus
extern "C" {
#endif

/* C++ module — string synthesis and dense number theory */
int cpp_strlen(const char *s);
long long cpp_factorial(int n);
int cpp_is_prime(int n);
long long cpp_primorial(int n);
int cpp_digit_sum(int n);

#ifdef __cplusplus
}
#endif

#endif
