#include <stdio.h>

// TODO: Define this function
int add_two_numbers(int a, int b) {
    return a + b;
}

#ifdef TEST_MODE
#include <assert.h>

void test_add_two_numbers() {
    assert(add_two_numbers(2, 3) == 5);
    assert(add_two_numbers(0, 0) == 0);
    assert(add_two_numbers(-1, 1) == 0);
    printf("All tests passed!\n");
}

int main() {
    test_add_two_numbers();
    return 0;
}
#else
int main() {
    printf("Result: %d\n", add_two_numbers(5, 3));
    return 0;
}
#endif