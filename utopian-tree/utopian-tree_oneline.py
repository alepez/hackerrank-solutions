import sys
print('\n'.join(map(lambda n : str(((1 << (((n-1) // 2) + 2)) - 2) + (not (n & 1))), map(int, sys.stdin.read().split()[1:]))))
