#include "swap.h"

void swap(int *const array, const int i, const int j) {
  int temp = array[i];
  array[i] = array[j];
  array[j] = temp;
}
