#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  std::vector<int> squares;
  for (int i = 0; i * i <= 1000000000; ++i) {
    squares.push_back(i * i);
  }
  int q, a, b;
  std::cin >> q;
  while (q--) {
    std::cin >> a >> b;
    auto ai = std::lower_bound(squares.begin(), squares.end(), a);
    auto bi = std::upper_bound(ai, squares.end(), b);
    std::cout << std::distance(ai, bi) << std::endl;
  }
}
