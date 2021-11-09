#include <assert.h>
#include <stdint.h>
#include <stdio.h>

int happy_ladybug(const char* b) {
  uint8_t d[26] = {0};
  uint8_t has_empty = 0;
  uint8_t already_happy = 1;
  uint8_t adj_count = 0;
  uint8_t left_c = 0;

  while (*b != 0) {
    uint8_t c = *b;

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

    ++b;
  }

  if ((adj_count < 2) && (left_c != 0)) {
    already_happy = 0;
  }

  uint8_t all_at_least_two = 1;

  for (uint8_t i = 0; i < 26; ++i) {
    if (d[i] == 1) {
      all_at_least_two = 0;
    }
  }

  return already_happy || (has_empty && all_at_least_two);
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
