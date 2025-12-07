#include <stdio.h>

int main() {
    int x = 10;
    int y = 15;
    {
        int x = 20;
        x += y;
        y += x;

        assert(x == ??); // What should this be?
        assert(y == ??); // What should this be?
    }
    assert(x == ??); // What should this be?
    assert(y == ??); // What should this be?
    return 0;
}