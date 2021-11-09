#include <assert.h>
#include <stdint.h>
#include <stdio.h>

int happy_ladybug(const char* b) {
  size_t d[26] = {0};
  size_t has_empty = 0;
  size_t already_happy = 1;
  size_t adj_count = 0;
  size_t left_c = 0;

  for (; *b != 0; ++b) {
    size_t c = *b;

    if (c == '_') {
      has_empty = 1;
    } else {
      d[c - 'A'] += 1;

      if (c == left_c) {
        adj_count += 1;
      }

      if ((adj_count < 2) && (left_c != 0)) {
        already_happy = 0;
      }

      if (c != left_c) {
        adj_count = 1;
      }

      left_c = c;
    }
  }

  if ((adj_count < 2) && (left_c != 0)) {
    already_happy = 0;
  }

  if (already_happy) {
    return 1;
  }

  for (size_t i = 0; i < 26; ++i) {
    if (d[i] == 1) {
      return 0;
    }
  }

  return has_empty;
}

int main() {
  assert(happy_ladybug("_") == 1);
  assert(happy_ladybug("AA") == 1);
  assert(happy_ladybug("A_BBA") == 1);
  assert(happy_ladybug("_ABBA") == 1);
  assert(happy_ladybug("AABB") == 1);
  assert(happy_ladybug("__AA") == 1);
  assert(happy_ladybug("BB__AA") == 1);
  assert(happy_ladybug("BB_AA") == 1);
  assert(happy_ladybug("BBAA_") == 1);
  assert(happy_ladybug("A") == 0);
  assert(happy_ladybug("_A") == 0);
  assert(happy_ladybug("AABBCD") == 0);
  assert(happy_ladybug("ABAB") == 0);
  assert(happy_ladybug("X_") == 0);
  assert(happy_ladybug("X_Y_Z_") == 0);
  assert(happy_ladybug("AABBC") == 0);
}

// def main():

// main()
