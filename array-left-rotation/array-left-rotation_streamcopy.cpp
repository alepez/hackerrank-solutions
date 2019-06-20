#include <iostream>
#include <vector>
using namespace std;
int main() {
  int n, d, i = 0;
  cin >> n >> d;
  /* skip first d numbers, keep them for later */
  vector<int> tmp(d);
  for (; i < d; ++i)
    cin >> tmp[i];
  /* skip the leading space */
  cin.get();
  /* copy remaining contents from in to out without parsing */
  cout << cin.rdbuf() << " ";

  for (int m : tmp)
    cout << m << " ";
}
