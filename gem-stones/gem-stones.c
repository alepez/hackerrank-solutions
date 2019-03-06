#include <string.h>
#include <stdio.h>

int main() {
  int n;
  scanf("%d", &n);
  char c;
  getchar();

  int f[26];
  memset(f, 0, sizeof(f));

  for (int i = 0; i != n; ++i) {
    int e[26];
    memset(e, 0, sizeof(e));

    /* Read all the string once, flag character when found */
    while (1) {
      c = getchar();
      if (c == '\n' || c == EOF) break;
      e[c - 'a'] |= 1;
    }

    /* Increment count of each found character */
    for (int i = 0; i != 26; ++i) {
      f[i] += e[i];
    }
  }

  int count = 0;
  for (int i = 0; i != 26; ++i) {
    count += f[i] == n;
  }

  printf("%d", count);
}
