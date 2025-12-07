#include <stdio.h>

// TODO: Add the missing argument `num` in the function definition.
void call_me(int num) {
    for (int i = 0; i < num; i++) {
        printf("Called!\n");
    }
}

int main() {
    // TODO: Fix the function call.
    call_me();
    return 0;
}