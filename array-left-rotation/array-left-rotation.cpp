#include <iostream>
#include <vector>
using namespace std;
int main() {
  int n, d, m;
  cin >> n >> d;
  vector<int> tmp;
  while (n--) {
    cin >> m;
    if (d-- > 0)
      tmp.push_back(m);
    else
      cout << m << " ";
  }
  for (int m : tmp)
    cout << m << " ";
}
