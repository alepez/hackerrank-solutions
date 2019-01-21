#include <iostream>

int main() {
  int n;
  std::cin >> n;
  while (n--) {
    int a, b, c;
    std::cin >> a >> b >> c;
    int da = std::abs(a - c);
    int db = std::abs(b - c);
    if (da == db) {
      std::cout << "Mouse C" << std::endl;
    } else if (da < db) {
      std::cout << "Cat A" << std::endl;
    } else {
      std::cout << "Cat B" << std::endl;
    }
  }
}
