#include <stdio.h>
#include <string.h>

int main() {
  int n;
  scanf("%d", &n);
  char c;
  getchar();

  int f[26];
  memset(f, 0, sizeof(f));

  for (int i = 0; i != n; ++i) {
    int e = 0;

    /* Read all the string once, flag character when found */
    while (1) {
      c = getchar();
      if (c == '\n' || c == EOF) break;
      e |= (1 << (c - 'a'));
    }

    /* Increment count of each found character */
    for (int i = 0; i != 26; ++i) {
      f[i] += (e & (1 << i)) ? 1 : 0;
    }
  }

  int count = 0;
  for (int i = 0; i != 26; ++i) {
    count += f[i] == n;
  }

  printf("%d", count);
}
