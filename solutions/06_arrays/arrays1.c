#include <stdio.h>
#include <assert.h>

int main() {
    
    // TODO: Define an array of length at least 5 and initialize it with values 0
    int arr[5] = {0};
    
    assert(arr[0] == 0);
    // TODO: Assign values to each index of the array other than 0
    for (int i = 1; i < 5; i++) {
        arr[i] = i;
    }
    assert(arr[0] != 0);
    return 0;
}