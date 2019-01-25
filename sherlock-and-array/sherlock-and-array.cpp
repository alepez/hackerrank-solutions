#include <algorithm>
#include <iostream>
#include <iterator>
#include <numeric>
#include <vector>

bool balancedOutput(const std::vector<int>& arr) {
  if (arr.size() == 1) return true;

  int sumL = 0;
  int sumR = std::accumulate(arr.begin() + 1, arr.end(), 0);

  for (unsigned i = 0; i != arr.size() - 1; ++i) {
    if (sumL == sumR) return true;
    sumL += arr[i];
    sumR -= arr[i + 1];
  }

  return false;
}

int main() {
  int t;
  std::cin >> t;
  while (t--) {
    int n;
    std::cin >> n;
    std::vector<int> arr(n);
    std::copy_n(std::istream_iterator<int>(std::cin), arr.size(), arr.begin());
    std::cout << (balancedOutput(arr) ? "YES\n" : "NO\n");
  }
}
