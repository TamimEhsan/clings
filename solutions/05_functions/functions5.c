#include <stdio.h>

// Define the function here to make the code compile and run correctly
int bar(int b);

// Do not change anything below this comment
int foo(int a) {
    return bar(a) + 5;
}

int bar(int b) {
    return b + b;
}

int main() {
    printf("%d\n", foo(3));
    return 0;
}