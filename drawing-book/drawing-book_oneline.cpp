#include <iostream>
int main() {
  int n, p;
  std::cin >> n >> p;
  std::cout << ((p <= (n / 2)) ? (p / 2) : ((n - p + !(n & 1)) / 2));
}
