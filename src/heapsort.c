#include "heapsort.h"
#include "swap.h"

void heapify(int *const array, const int h, const int i) {
    int l = (2 * i) + 1;
    int r = (2 * i) + 2;

    int max = 0;
    if (l < h && array[l] > array[i]) {
        max = l;
    } else {
        max = i;
    }
    if (r < h && array[r] > array[max]) {
        max = r;
    }
    if (max != i) {
        swap(array, i, max);
        heapify(array, h, max);
    }
}

void build_heap(int *const array, const int size) {
    const int h = size;
    for (int i = (h / 2); i > 0; i--) {
        heapify(array, h, i - 1);
    }
}

void heapsort(int *const array, const int size) {
    build_heap(array, size);
    for (int i = (size - 1); i > 0; i--) {
        swap(array, 0, i);
        heapify(array, i, 0);
    }
}
