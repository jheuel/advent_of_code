#!/usr/bin/env python3

import re
from collections import defaultdict


def read_input(fn):
    with open(fn) as f:
        return [int(i) for i in f.read().strip('\n').split(',')]


def run(inp, N):
    last = defaultdict(list)
    for t, i in enumerate(inp):
        last[i].append(t)
        nlast = i

    for t in range(len(inp), N):
        last[nlast].append(t-1)
        if last[nlast][0] == t - 1:
            n = 0
        else:
            n = last[nlast][-1] - last[nlast][-2]
        nlast = n
    print(n)


def solve1(inp):
    run(inp, 2020)


def solve2(inp):
    run(inp, 30000000)


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)

    solve1(inp)
    solve2(inp)
