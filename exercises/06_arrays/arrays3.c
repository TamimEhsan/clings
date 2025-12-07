#include <stdio.h>
#include <assert.h>

// TODO: What will be the function signature?
int sumArray(??, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

int main() {
    int arr[] = {1, 2, 3, 4, 5};
    int total = sumArray(??, 5);
    assert(total == 15);
    return 0;
}