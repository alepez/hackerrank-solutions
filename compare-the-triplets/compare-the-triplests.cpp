#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>

using namespace std;

array<int, 2> compare_triplets(array<int, 3> a, array<int, 3> b) {
  array<int, 2> points{};
  for (size_t i = 0; i < a.size(); i++) {
    points[a[i] < b[i]] += a[i] != b[i];
  }
  return points;
}

int main() {

  array<int, 3> a, b;
  copy_n(istream_iterator<int>(cin), 3, begin(a));
  copy_n(istream_iterator<int>(cin), 3, begin(b));
  auto points = compare_triplets(a, b);
  std::cout << points[0] << " " << points[1] << std::endl;
}
