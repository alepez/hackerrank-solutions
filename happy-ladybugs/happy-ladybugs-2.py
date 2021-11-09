#!/bin/python3

def happy_ladybug(b):
    d = {}
    has_empty = False
    already_happy = True
    adj_count = 0
    left_c = None

    for c in b:
        if c == '_':
            has_empty = True
        else:
            d[c] = d.setdefault(c, 0) + 1

            if c == left_c:
                adj_count += 1

            if (adj_count < 2) and (left_c is not None):
                already_happy = False

            if c != left_c:
                adj_count = 1

            left_c = c

    if (adj_count < 2) and (left_c is not None):
        already_happy = False

    all_at_least_two = all([x >= 2 for x in d.values()])

    return already_happy or (has_empty and all_at_least_two)


def main():
    assert happy_ladybug("_")
    assert happy_ladybug("AA")
    assert happy_ladybug("A_BBA")
    assert happy_ladybug("_ABBA")
    assert happy_ladybug("AABB")
    assert happy_ladybug("__AA")
    assert happy_ladybug("BB__AA")
    assert happy_ladybug("BB_AA")
    assert happy_ladybug("BBAA_")
    assert not happy_ladybug("A")
    assert not happy_ladybug("_A")
    assert not happy_ladybug("AABBCD")
    assert not happy_ladybug("ABAB")
    assert not happy_ladybug("X_")
    assert not happy_ladybug("X_Y_Z_")
    assert not happy_ladybug("AABBC")


main()
