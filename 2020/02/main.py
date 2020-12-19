#!/usr/bin/env python3

import re
from collections import Counter


def read_input(fn):
    r = re.compile('(.*)-(.*) (.): (.*)')
    with open(fn) as f:
        lines = [r.findall(i)[0] for i in f.read().split('\n') if len(i) > 0]
        return lines


def solve1(inp):
    valid = 0
    invalid = 0
    for i in inp:
        _min, _max = int(i[0]), int(i[1])
        letter = i[2]
        c = Counter(i[3])
        if c[letter] < _min or c[letter] > _max:
            invalid += 1
            continue
        valid += 1
    print(f'valid after first rule: {valid}')


def solve2(inp):
    valid = 0
    invalid = 0
    for i in inp:
        _min, _max = int(i[0]), int(i[1])
        letter = i[2]
        c = Counter(i[3][_min - 1:_min] + i[3][_max - 1:_max])
        if c[letter] != 1:
            invalid += 1
            continue
        valid += 1
    print(f'valid after second rule: {valid}')


for fn in ['test', 'input']:
    print(fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
    print()
