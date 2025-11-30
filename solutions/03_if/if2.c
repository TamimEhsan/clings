#include <stdio.h>
#include <string.h>

int map_animal_to_legs(const char* animal) {
    // TODO: complete the branching to get correct assertions
    if (strcmp(animal, "spider") == 0) {
        return 8;
    } else if (strcmp(animal, "dog") == 0) {
        return 4;
    } else if (strcmp(animal, "human") == 0) {
        return 2;
    } else {
        return 0;
    }
}

#ifdef TEST_MODE
#include <assert.h>

void test_map_animal_to_legs() {
    assert(map_animal_to_legs("spider") == 8);
    assert(map_animal_to_legs("dog") == 4);
    assert(map_animal_to_legs("human") == 2);
    assert(map_animal_to_legs("unknown") == 0);
    printf("All tests passed!\n");
}

int main() {
    test_map_animal_to_legs();
    return 0;
}
#else
int main() {
    printf("Result: %d\n", map_animal_to_legs("dog"));
    return 0;
}
#endif