#include <stdio.h>
#include <assert.h>

// TODO: What will be the function signature?
int sumMatrix(int arr[][5], int n, int m) {
    int sum = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            sum += arr[i][j];
        }
    }
    return sum;
}

int main() {
    int arr[3][5] = {
        {1, 2, 3, 4, 5},
        {6, 7, 8, 9, 10},
        {11, 12, 13, 14, 15}
    };
    int total = sumMatrix(arr, 3, 5);
    assert(total == 120);
    return 0;
}