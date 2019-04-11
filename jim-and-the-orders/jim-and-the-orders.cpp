#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  struct Customer {
    int i;
    int orderNumber;
    int prepTime;
  };

  std::vector<Customer> orders;
  int n;
  std::cin >> n;

  for (int i = 1; i <= n; ++i) {
    Customer c;
    c.i = i;
    std::cin >> c.orderNumber;
    std::cin >> c.prepTime;
    orders.push_back(c);
  }

  std::sort(orders.begin(), orders.end(), [](Customer l, Customer r) {
    const int diff = l.orderNumber - r.orderNumber + l.prepTime - r.prepTime;
    return diff == 0 ? l.i < r.i : diff < 0;
  });

  for (auto c : orders) {
    std::cout << c.i << " ";
  }
}
