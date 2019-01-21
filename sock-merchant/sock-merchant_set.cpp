#include <iostream>
#include <set>

int main() {
  int n;
  std::cin >> n;
  std::set<int> m;
  int sum = 0;
  while (n--) {
    int c;
    std::cin >> c;
    if (m.count(c)) {
      ++sum;
      m.erase(c);
    } else {
      m.insert(c);
    }
  }
  std::cout << sum << std::endl;
}
