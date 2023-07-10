#include "mergesort.h"

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

  print_array(array, size);
  int *sorted = mergesort(array, size, false);
  print_array(sorted, size);
  free(sorted);

  return 0;
}
