#include "heapsort.h"
#include "mergesort.h"
#include "quicksort.h"

#include <stdio.h>
#include <stdlib.h>

void print_array(const int array[], const int size) {
    printf("[ ");
    for (int i = 0; i < size; i++) {
        if (i < (size - 1)) {
            printf("%d, ", array[i]);
        } else {
            printf("%d", array[i]);
        }
    }
    printf(" ]\n");
}

int main(int argc, const char *argv[]) {
    int array[] = {1, 9, 0, 5, 6, 7, 8, 2, 4, 3};
    const int size = sizeof(array) / sizeof(int);

    // heapsort
    /*
    print_array(array, size);
    heapsort(array, size);
    print_array(array, size);
    */

    // quicksort
    /*
    print_array(array, size);
    quicksort(array, 0, size - 1);
    print_array(array, size);
    */

    // mergesort
    print_array(array, size);
    int *sorted = mergesort(array, size);
    print_array(sorted, size);
    free(sorted);

    return 0;
}
