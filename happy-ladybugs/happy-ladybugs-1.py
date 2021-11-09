#!/bin/python3

def happy_ladybug(b):
    if len(b) == 0:
        return True

    left_c = b[0]
    b = b[1:]
    d = {}
    has_empty = left_c == '_'
    already_happy = True
    adj_count = 1

    if left_c != '_':
        d[left_c] = d.setdefault(left_c, 0) + 1

    for c in b:
        if c == '_':
            has_empty = True
            if adj_count < 2:
                already_happy = False
        else:
            d[c] = d.setdefault(c, 0) + 1

            if c == left_c:
                adj_count += 1
            else:
                if adj_count < 2:
                    already_happy = False
                adj_count = 1
            left_c = c

    if adj_count < 2:
        already_happy = False

    all_at_least_two = all([x >= 2 for x in d.values()])

    return already_happy or (has_empty and all_at_least_two)


def main():
    assert happy_ladybug("")
    assert happy_ladybug("A_BBA")
    assert happy_ladybug("_ABBA")
    assert happy_ladybug("AABB")
    assert happy_ladybug("__AA")
    assert happy_ladybug("BB__AA")
    assert happy_ladybug("BB_AA")
    assert happy_ladybug("BBAA_")
    assert not happy_ladybug("ABAB")
    assert not happy_ladybug("X_")
    assert not happy_ladybug("X_Y_Z_")
    assert not happy_ladybug("AABBC")


main()
