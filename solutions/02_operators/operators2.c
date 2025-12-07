#include <stdio.h>
#include <assert.h>

int main() {
    int a = 10;
    int b = 3;
    int sum = a + b; // Addition
    int diff = a - b; // Subtraction
    int prod = a * b; // Multiplication
    int quot = a / b; // Division
    int mod = a % b; // Modulus

    assert(sum == 13);
    assert(diff == 7);
    assert(prod == 30);
    assert(quot == 3);
    assert(mod == 1);
    return 0;
}