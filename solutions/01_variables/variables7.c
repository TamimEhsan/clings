#include <stdio.h>

int main() {
    int x = 10;
    int y = 15;
    {
        int x = 20;
        x += y;
        y += x;

        assert(x == 35); // What should this be?
        assert(y == 50); // What should this be?
    }
    assert(x == 10); // What should this be?
    assert(y == 50); // What should this be?
    return 0;
}