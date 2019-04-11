#include <iostream>
#include <vector>

char getNeighbour(const std::vector<std::vector<char>> &cells, int i, int j) {
  return i >= 0 && i < cells.size() && j >= 0 && j < cells.size() ? cells[i][j]
                                                                  : 0;
}

char maxNeighbour(const std::vector<std::vector<char>> &cells, int i, int j) {
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
  std::cin.get();

  std::vector<std::vector<char>> grid;

  while (std::cin) {
    std::vector<char> row;

    while (std::cin) {
        char c = std::cin.get();
        if (c == '\n' || !std::cin) break;
        row.push_back(c);
    }

    grid.push_back(row);
  }

  auto gridCopy = grid;

  for (int i = 1; i < grid.size() - 1; ++i) {
    for (int j = 1; j < grid[i].size() - 1; ++j) {
      int max = maxNeighbour(grid, i, j);
      if (grid[i][j] > max) {
        gridCopy[i][j] = 'X';
      }
    }
  }

  for (auto& row : gridCopy) {
    for (auto& cell : row) {
      std::cout << cell;
    }
    std::cout << "\n";
  }
}

