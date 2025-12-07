#include <stdio.h>
#include <assert.h>
#include <stdarg.h>

// Implement the function to calculate the average of given numbers
double average(int count, ...) {
    // Your code here
}

int main() {
    double avg1 = average(3, 10, 20, 30);
    double avg2 = average(5, 5, 15, 25, 35, 45);

    assert(avg1 == 20.0);
    assert(avg2 == 25.0);
    return 0;
}