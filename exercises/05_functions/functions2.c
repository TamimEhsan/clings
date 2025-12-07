#include <stdio.h>

// TODO: Add the missing argument `num` in the function definition.
void call_me() {
    for (int i = 0; i < num; i++) {
        printf("Called!\n");
    }
}

int main() {
    call_me(5);
    return 0;
}