#include <stdio.h>

int bigger(int a, int b) {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if (a > b) {
        return a;
    }
    return b;
}

#ifdef TEST_MODE
#include <assert.h>

void test_bigger() {
    assert(bigger(2, 3) == 3);
    assert(bigger(0, 0) == 0);
    assert(bigger(10, -10) == 10);
    printf("All tests passed!\n");
}

int main() {
    test_bigger();
    return 0;
}
#else
int main() {
    printf("Result: %d\n", bigger(5, 3));
    return 0;
}
#endif