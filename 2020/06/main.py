#!/usr/bin/env python3


def read_input(fn):
    print(fn)
    with open(fn) as f:
        return [
            i.strip('\n').split('\n') for i in f.read().split('\n\n')
            if len(i) > 0
        ]


def solve1(inp):
    print('part1:')
    sum_of_counts = 0
    for group in inp:
        sum_of_counts += len(set(''.join(group)))
    print(f'sum of counts: {sum_of_counts}')


def solve2(inp):
    print('part2:')
    sum_of_counts = 0
    for group in inp:
        members = [set(i) for i in group]
        intersection = set.intersection(*members)
        sum_of_counts += len(intersection)
    print(f'sum of counts: {sum_of_counts}')


for fn in ['test', 'input']:
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
    print('\n')
