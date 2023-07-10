#include "quicksort.h"

#include <stdio.h>

static void swap(int *const array, const int i, const int j) {
  int temp = array[i];
  array[i] = array[j];
  array[j] = temp;
}

static int partition(int *const array, const int p, const int r) {
  int x = array[r];
  int i = p - 1;
  for (int j = p; j <= (r - 1); j++) {
    if (array[j] <= x) {
      swap(array, ++i, j);
    }
  }
  swap(array, i + 1, r);
  return i + 1;
}

void quicksort(int *const array, const int p, const int r) {
  if (p < r) {
    const int q = partition(array, p, r);
    quicksort(array, p, q - 1);
    quicksort(array, q + 1, r);
  }
}