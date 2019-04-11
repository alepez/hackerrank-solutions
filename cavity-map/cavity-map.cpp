#include <iostream>
#include <vector>

char getNeighbour(const std::vector<std::vector<char>>& cells, int i, int j) {
  return i >= 0 && i < cells.size() && j >= 0 && j < cells.size() ? cells[i][j] : 0;
}

char maxNeighbour(const std::vector<std::vector<char>>& cells, int i, int j) {
  char max = 0;
  max = std::max(max, getNeighbour(cells, i - 1, j));
  max = std::max(max, getNeighbour(cells, i + 1, j));
  max = std::max(max, getNeighbour(cells, i, j - 1));
  max = std::max(max, getNeighbour(cells, i, j + 1));
  return max;
}

int main() {
  int n;
  std::cin >> n;
  std::vector<std::vector<char>> cells;
  cells.resize(n);
  std::cin.get();

  for (int i = 0; i != n; ++i) {
    cells[i].resize(n);
    for (int j = 0; j != n; ++j) {
      cells[i][j] = ((char)std::cin.get());
    }
    std::cin.get();
  }

  auto cellsCopy = cells;

  for (int i = 1; i != n - 1; ++i) {
    for (int j = 1; j != n - 1; ++j) {
      int max = maxNeighbour(cells, i, j);
      if (cells[i][j] > max) {
        cellsCopy[i][j] = 'X';
      }
    }
  }

  for (int i = 0; i != n; ++i) {
    for (int j = 0; j != n; ++j) {
      std::cout << cellsCopy[i][j];
    }
    std::cout << "\n";
  }
}
