#include <iostream>
#include <array>
#include <algorithm>

int main() {
  int n;
  std::cin >> n;
  char c;
  std::cin.get(c); /* This is needed to consume the newline character */

  std::array<int, 26> f{};

  for (int i = 0; i != n; ++i) {
    std::array<bool, 26> e{};

    /* Read all the string once, flag character when found */
    while (std::cin)  {
      std::cin.get(c);
      if (c == '\n') break;
      e[c - 'a'] |= true;
    }

    /* Increment count of each found character */
    for (int i = 0; i != 26; ++i) {
      f[i] += e[i];
    }
  }

  std::cout << std::count(f.begin(), f.end(), n) << std::endl;
}
