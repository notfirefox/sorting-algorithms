#include "mergesort.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int *const merge(const int *const array_1, const int *const array_2,
                 const int size_1, const int size_2) {
  int *result = malloc(sizeof(int) * (size_1 + size_2));
  if (result == NULL) {
    printf("Memory allocation failed.\n");
    exit(EXIT_FAILURE);
    return NULL;
  }
  int index_1 = 0;
  int index_2 = 0;
  int index_3 = 0;
  while (index_1 < size_1 && index_2 < size_2) {
    if (array_1[index_1] <= array_2[index_2]) {
      result[index_3++] = array_1[index_1++];
    } else {
      result[index_3++] = array_2[index_2++];
    }
  }
  if (index_1 < size_1) {
    while (index_1 < size_1) {
      result[index_3++] = array_1[index_1++];
    }
  } else if (index_2 < size_2) {
    while (index_2 < size_2) {
      result[index_3++] = array_2[index_2++];
    }
  }
  return result;
}

int *const _mergesort(int *const array, const int size, bool should_free) {
  if (size <= 1) {
    return array;
  }

  const int mid = size / 2;

  const int size_1 = mid;
  int *array_1 = malloc(sizeof(int) * size_1);
  if (array_1 == NULL) {
    printf("Memory allocation failed.\n");
    exit(EXIT_FAILURE);
    return NULL;
  }
  memcpy(array_1, array, sizeof(int) * size_1);

  const int size_2 = size - mid;
  int *array_2 = malloc(sizeof(int) * size_2);
  if (array_2 == NULL) {
    printf("Memory allocation failed.\n");
    exit(EXIT_FAILURE);
    return NULL;
  }
  memcpy(array_2, array + mid, sizeof(int) * size_2);

  _mergesort(array_1, size_1, true);
  _mergesort(array_2, size_2, true);

  if (should_free) {
    free(array);
  }

  int *const result = merge(array_1, array_2, size_1, size_2);
  free(array_1);
  free(array_2);

  return result;
}

int *const mergesort(int *const array, const int size) {
  return _mergesort(array, size, false);
}
