#include <iostream>

int main() {
  int t;
  std::cin >> t;
  while (t--) {
    int n, k;
    std::cin >> n >> k;
    int ok = 0;
    while (n--) {
      int a;
      std::cin >> a;
      ok += a <= 0;
    }
    std::cout << ((ok < k) ? "YES" : "NO") << "\n";
  }
}
