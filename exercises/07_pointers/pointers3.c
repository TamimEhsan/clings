#include <stdio.h>
#include <assert.h>

// Fix the function to swap the values of x and y
void swap(int x, int y) {
    int temp = x;
    x = y;
    y = temp;
}

int main() {
    int a = 5;
    int b = 10;
    swap(a, b); // You might need to change something here too
    assert(a == 10);
    assert(b == 5);
    return 0;
} 