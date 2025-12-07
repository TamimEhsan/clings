#include <stdio.h>
#include <assert.h>

int main() {
    int a = 1;
    int b = 2;
    int c = 3;
    int result = a + b * c; // Add parentheses to change the precedence

    assert(result == 9);
    return 0;
}