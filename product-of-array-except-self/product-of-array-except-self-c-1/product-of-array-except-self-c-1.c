#include <stdio.h>
#include <stdlib.h>

void solve_in_place_with_div(int* arr, int n) {
  int product = 1;

  for (int i = 0; i < n; ++i) {
    product *= arr[i];
  }

  for (int i = 0; i < n; ++i) {
    arr[i] = product / arr[i];
  }
}

void solve_in_place_with_prefix_product(int* arr, int n) {
  int* pp = (int*)malloc(sizeof(int) * n);

  int acc = 1;

  pp[0] = acc;

  for (int i = 1; i < n; ++i) {
    acc *= arr[i - 1];
    pp[i] = acc;
  }

  acc = arr[n - 1];

  acc = 1;

  for (int i = 0; i < n; ++i) {
    int tmp = arr[n - i - 1];
    arr[n - i - 1] = pp[n - i - 1] * acc;
    acc *= tmp;
  }

  free(pp);
}

int main() {
  int n = 0;
  scanf("%d", &n);

  int* arr = (int*)malloc(sizeof(int) * n);

  for (int i = 0; i < n; ++i) {
    scanf("%d", &arr[i]);
  }

  solve_in_place_with_prefix_product(arr, n);

  for (int i = 0; i < n; ++i) {
    printf("%d ", arr[i]);
  }

  free(arr);
  return 0;
}
