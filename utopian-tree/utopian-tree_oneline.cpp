#include <iostream>

int main() {
  int n;
  std::cin >> n;
  while (n--) {
    int c;
    std::cin >> c;
    std::cout
      << (c == 0 ? 1 : ((1 << (((c - 1) / 2) + 2)) - 2) + (!(c & 1)))
      << "\n";
  }
}
