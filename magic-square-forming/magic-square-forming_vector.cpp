#include <bits/stdc++.h>

using namespace std;

using Square = std::vector<std::vector<int>>;
using Squares = std::vector<Square>;

Square rotate(const Square& sq0) {
  auto sq1 = sq0;
  sq1[2][0] = sq0[0][0];
  sq1[1][0] = sq0[0][1];
  sq1[0][0] = sq0[0][2];
  sq1[2][1] = sq0[1][0];
  sq1[1][1] = sq0[1][1];
  sq1[0][1] = sq0[1][2];
  sq1[2][2] = sq0[2][0];
  sq1[1][2] = sq0[2][1];
  sq1[0][2] = sq0[2][2];
  return sq1;
}

Square mirror(const Square& sq0) {
  auto sq1 = sq0;
  sq1[0][0] = sq0[2][0];
  sq1[0][1] = sq0[2][1];
  sq1[0][2] = sq0[2][2];
  sq1[1][0] = sq0[1][0];
  sq1[1][1] = sq0[1][1];
  sq1[1][2] = sq0[1][2];
  sq1[2][0] = sq0[0][0];
  sq1[2][1] = sq0[0][1];
  sq1[2][2] = sq0[0][2];
  return sq1;
}

static const Squares allMagicSquares = [] {
  Square baseMagicSquare = {
      {2, 7, 6},
      {9, 5, 1},
      {4, 3, 8},
  };
  Squares squares;
  squares.push_back(baseMagicSquare);
  squares.push_back(rotate(squares.back()));
  squares.push_back(rotate(squares.back()));
  squares.push_back(rotate(squares.back()));
  squares.push_back(mirror(squares[0]));
  squares.push_back(mirror(squares[1]));
  squares.push_back(mirror(squares[2]));
  squares.push_back(mirror(squares[3]));
  return squares;
}();

int calculateCost(const Square& from, const Square& target) {
  int cost = 0;
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      cost += std::abs(from[i][j] - target[i][j]);
    }
  }
  return cost;
}

// Complete the formingMagicSquare function below.
int formingMagicSquare(vector<vector<int>> sq0) {
  int m = 999;
  for (auto& sq1 : allMagicSquares) {
    m = std::min(m, calculateCost(sq0, sq1));
  }
  return m;
}

int main() {
  std::ofstream fout(getenv("OUTPUT_PATH"));

  Square s(3);
  for (int i = 0; i < 3; i++) {
    s[i].resize(3);

    for (int j = 0; j < 3; j++) {
      std::cin >> s[i][j];
    }

    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
  }

  int result = formingMagicSquare(s);

  fout << result << "\n";

  fout.close();

  return 0;
}
