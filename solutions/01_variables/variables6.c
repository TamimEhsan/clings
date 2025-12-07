#include <stdio.h>
#include <assert.h>

int count = 0;
// Fix the code to correctly compile and run
// without changing anything below this comment
void increment() {
    count++;
}

void assertCount() {
    assert(count == 1);
}

int main() {
    int count = 5;
    increment();
    assertCount();
    assert(count == 5);
    return 0;
}