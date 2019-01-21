#include <iostream>
#include <array>
#include <iterator>
#include <algorithm>

int main() {
  std::array<int, 26> heights;
  std::copy_n(std::istream_iterator<int>(std::cin), heights.size(), heights.begin());
  char c;
  int max = 0;
  int count = 0;
  while (std::cin.get(c)) {
    if ('a' <= c && c <= 'z') {
      max = std::max(max, heights[c - 'a']);
      ++count;
    }
  }
  std::cout << (count * max);
}
