#include <stdio.h>

int main() {
    bool is_valid = true;
    printf("is_valid: %d\n", is_valid);

    int value1 = is_valid;
    int value2 = !is_valid;

    // TODO: Fix the assertion to check the correct value of 'value'.
    assert(value1 == 1);
    assert(value2 == 0);

}