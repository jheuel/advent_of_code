#!/usr/bin/env python3

from collections import Counter


def read_input(fn):
    with open(fn) as f:
        inp = [int(i.strip('\n')) for i in f.readlines()]
        inp.append(0)
        inp.append(max(inp) + 3)
        inp.sort()
        return inp


def solve1(inp):
    c = Counter([i - j for i, j in zip(inp[1:], inp[:-1])])
    ones = c[1]
    threes = c[3]
    print(f'{ones} ones and {threes} threes, multiplied: {ones * threes}')


def solve2(inp):
    paths = [1] * len(inp)
    for i in range(len(inp) - 4, -1, -1):
        paths[i] = paths[i + 1]
        if inp[i + 2] - inp[i] < 4:
            paths[i] += paths[i + 2]
        if inp[i + 3] - inp[i] < 4:
            paths[i] += paths[i + 3]
    print(f'possible paths: {paths[0]}')


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
