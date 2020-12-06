#!/usr/bin/env python

import re


def read_input(fn):
    with open(fn) as f:
        seats = [i for i in f.read().split('\n') if len(i) > 0]
        return seats


def from_code(code):
    binary = {'F': '0', 'B': '1', 'L': '0', 'R': '1'}
    return int(''.join([binary[i] for i in code]), 2)


def solve1(fn):
    print(fn)
    print('part1:')
    inp = read_input(fn)

    valid = re.compile('^([FB]{7})([LR]{3})$')

    ids = []
    for code in inp:
        match = valid.match(code)
        if match is None:
            raise ValueError(f'wrong format: {code}')
        ids.append(from_code(code))

    print(f'highest seat ID is {max(ids)}')
    return ids


def solve2(ids):
    print('part2:')
    seats = [i for i in range(min(ids), max(ids)) if i not in ids]
    print(f'available seats: {seats}')


for fn in ['test', 'input']:
    ids = solve1(fn)
    solve2(ids)
    print('\n')
