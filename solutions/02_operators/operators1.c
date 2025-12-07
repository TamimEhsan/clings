#include <stdio.h>
#include <assert.h>

int main() {
    int a = 4;
    int b = 7;
    int c = (++a) * (b--); // Add unary operators here to make the assert work
    assert(a == 5);
    assert(b == 6);
    assert(c == 35);
    return 0;
}