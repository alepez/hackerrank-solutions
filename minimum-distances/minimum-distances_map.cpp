#include <iostream>
#include <map>

int main() {
  std::map<int, int> pos;
  std::map<int, int> dist;

  int n;
  std::cin >> n;

  for (int i = 0; i != n; ++i) {
    int a;
    std::cin >> a;

    if (pos.count(a)) {
      if (dist.count(a)) {
        dist[a] = std::min(dist[a], i - pos[a]);
      } else {
        dist[a] = i - pos[a];
      }
    }

    pos[a] = i;
  }

  if (dist.empty()) {
    std::cout << -1 << std::endl;
  } else {
    auto it = dist.begin();
    int min = it->second;
    ++it;
    for (; it != dist.end(); ++it) {
      min = std::min(min, it->second);
    }
    std::cout << min << std::endl;
  }
}
