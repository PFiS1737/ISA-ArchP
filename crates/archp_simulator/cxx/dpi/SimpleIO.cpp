#include <cstdio>

#include "Vtop__Dpi.h"

int simple_input() {
  int32_t value;
  printf("Input: ");
  scanf("%d", &value);
  return value;
}

void simple_output(int value) {
  printf("Output: %d\n", value);
}
