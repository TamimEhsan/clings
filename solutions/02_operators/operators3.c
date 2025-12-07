#include <stdio.h>
#include <assert.h>

int main() {
    int a = 10;
    int b = 20;
    int max = (a > b) ? a : b; // Use ternary operator to find max

    assert(max == 20);
    return 0;
}