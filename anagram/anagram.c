#include <stdio.h>
#include <stdlib.h>

int count_changes(char* string, unsigned size) {
  if (size & 1) {
    return -1;
  }

  int freq[26] = {};

  size_t const half = size / 2;

  for (size_t i = 0; i < half; ++i) {
    ++freq[(size_t)string[i] - 'a'];
  }

  for (size_t i = half; i < size; ++i) {
    --freq[(size_t)string[i] - 'a'];
  }

  int sum = 0;

  for (size_t i = 0; i < 26; ++i) {
    sum += freq[i] > 0 ? freq[i] : 0;
  }

  return sum;
}

int main() {
  int n;
  scanf("%i\n", &n);
  char* string = (char*)malloc(10001 * sizeof(char));

  for (int i = 0; i < n; ++i) {
    size_t size = 10001;
    size = getline((char**)&string, &size, stdin);

    /* Fix strings ending with space or newline */
    while (string[size - 1] == ' ' || string[size - 1] == '\n') {
      size -= 1;
    }

    printf("%i\n", count_changes(string, size));
  }

  free(string);
  return 0;
}
