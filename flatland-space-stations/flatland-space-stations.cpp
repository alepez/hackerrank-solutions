#include <algorithm>
#include <iostream>
#include <limits>
#include <vector>

int maximum_distance_to_space_station(int n, std::vector<int> space_stations) {
  std::sort(space_stations.begin(), space_stations.end());

  std::vector<int> distances;
  distances.resize(n);
  std::fill(distances.begin(), distances.end(), std::numeric_limits<int>::max());

  {
    int last = space_stations.front();
    for (auto it = 1 + space_stations.begin(); it != space_stations.end(); ++it) {
      int d = 0;
      for (int i = last; i < *it; ++i) {
        distances[i] = std::min(d, distances[i]);
        ++d;
      }
      last = *it;
    }
  }

  {
    int d = 0;
    for (int i = space_stations.back(); i < n; ++i) {
      distances[i] = std::min(d, distances[i]);
      ++d;
    }
  }

  {
    int last = space_stations.back();
    for (auto it = 1 + space_stations.rbegin(); it != space_stations.rend(); ++it) {
      int d = 0;
      for (int i = last; i > *it; --i) {
        distances[i] = std::min(d, distances[i]);
        ++d;
      }
      last = *it;
    }
  }

  {
    int d = 0;
    for (int i = space_stations.front(); i >= 0; --i) {
      distances[i] = std::min(d, distances[i]);
      ++d;
    }
  }

  return *std::max_element(distances.begin(), distances.end());
}

int main() {
  int n;
  int m;
  std::cin >> n;
  std::cin >> m;
  std::vector<int> space_stations;
  space_stations.reserve(m);
  while (m--) {
    int s;
    std::cin >> s;
    space_stations.push_back(s);
  }

  std::cout << maximum_distance_to_space_station(n, space_stations) << std::endl;
}
