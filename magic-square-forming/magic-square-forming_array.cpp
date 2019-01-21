#include <array>
#include <iostream>
#include <limits>

using Row = std::array<int, 3>;
using Square = std::array<Row, 3>;
using Squares = std::array<Square, 8>;

Square rotate(Square sq0) {
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

Square mirror(Square sq0) {
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

Squares generateMagicSquares() {
  Square sq = {
      Row{2, 7, 6},
      Row{9, 5, 1},
      Row{4, 3, 8},
  };
  Squares squares;
  squares[0] = sq;
  squares[1] = rotate(squares[0]);
  squares[2] = rotate(squares[1]);
  squares[3] = rotate(squares[2]);
  squares[4] = mirror(squares[0]);
  squares[5] = mirror(squares[1]);
  squares[6] = mirror(squares[2]);
  squares[7] = mirror(squares[3]);
  return squares;
}

static const auto allMagicSquares = generateMagicSquares();

int calculateCost(Square from, Square target) {
  int cost = 0;
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      cost += std::abs(from[i][j] - target[i][j]);
    }
  }
  return cost;
}

int formingMagicSquare(Square sq0) {
  int m = 999;
  for (auto& sq1 : allMagicSquares) {
    m = std::min(m, calculateCost(sq0, sq1));
  }
  return m;
}

int main() {
  Square s;
  for (int i = 0; i < 3; i++) {
    for (int j = 0; j < 3; j++) {
      std::cin >> s[i][j];
    }
  }
  std::cout << formingMagicSquare(s);
}
