#include <iostream>
#include <map>

int main() {
  std::array<int, 100001> pos;
  pos.fill(-1);
  std::array<int, 100001> dist;
  dist.fill(0x7FFFFFFF);

  int n;
  std::cin >> n;

  for (int i = 0; i != n; ++i) {
    int a;
    std::cin >> a;

    if (pos[a] != -1) {
      if (dist[a] != -1) {
        dist[a] = std::min(dist[a], i - pos[a]);
      } else {
        dist[a] = i - pos[a];
      }
    }

    pos[a] = i;
  }

  int min = dist[0];
  for (int i = 1; i != dist.size(); ++i) {
    min = std::min(dist[i], min);
  }

  std::cout << (min == 0x7FFFFFFF ? -1 : min) << std::endl;
}
