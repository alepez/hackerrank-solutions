#include <iostream>
#include <vector>

int main() {
  std::vector<std::string> s;

  std::string d;

  while (std::cin) {
    char c = std::cin.get();

    if (c == '/' || c == '\n' || c == EOF) {

      if (d == "..") {
        if (!s.empty()) {
          s.pop_back();
        }
      } else if (d != "." && !d.empty()) {
        s.push_back(std::move(d));
      }

      d.clear();
    } else {
      d.push_back(c);
    }
  }

  if (s.empty()) {
    std::cout << "/";
  } else {
    for (auto t : s) {
      std::cout << "/" << t;
    }
  }
}
