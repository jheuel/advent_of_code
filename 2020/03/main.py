#!/usr/bin/env python3


def read_input(fn):
    with open(fn) as f:
        lines = [i for i in f.read().split('\n') if len(i) > 0]
        return lines


def solve1(fn, slope):
    print(fn, end=': ')
    inp = read_input(fn)
    xmax = len(inp[0])
    ymax = len(inp)
    x, y = [0, 0]
    trees = 0
    while y < len(inp):
        x += slope[0]
        y += slope[1]

        if y >= ymax:
            break
        if inp[y][x % xmax] == '#':
            trees += 1
    print(f'trees for slope {slope}: {trees}')
    return trees


slopes = {
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
}

for fn in ['test', 'input']:
    print(fn)
    product = 1
    for slope in slopes:
        product *= solve1(fn, slope)
    print(f'product: {product}\n')
