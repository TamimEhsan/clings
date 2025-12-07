#include <stdio.h>
#include <assert.h>


int main() {
    int arr[] = {1, 2, 3, 4, 5};
    int total = 0;
    // TODO: Calculate the sum of array elements
    for (int i = 0; i < 5; i++) {
        total += arr[i];
    }

    assert(total == 15);
    return 0;
}