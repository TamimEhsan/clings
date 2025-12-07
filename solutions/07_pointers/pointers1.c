#include <stdio.h>
#include <assert.h>

int main() {
    int a = 5;
    int *p = &a; // Fix this line to declare and initialize a pointer to a
    a = 6;

    assert(*p == 6);
    return 0;
}