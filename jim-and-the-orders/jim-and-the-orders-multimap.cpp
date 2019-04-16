#include <algorithm>
#include <iostream>
#include <map>

int main() {
  std::multimap<int, int> orders;
  int n;
  std::cin >> n;

  for (int i = 1; i <= n; ++i) {
    int orderNumber, prepTime;
    std::cin >> orderNumber >> prepTime;
    orders.insert(std::make_pair(orderNumber + prepTime, i));
  }

  for (auto o : orders) {
    std::cout << o.second << " ";
  }
}
