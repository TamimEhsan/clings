#include <stdio.h>
#include <assert.h>

int main() {
    int arr[] = {10, 20, 30};
    int *p = arr;
    int secondElement;
    assert(secondElement == 20);
    return 0;
}  