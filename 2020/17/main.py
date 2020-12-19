#!/usr/bin/env python3

import itertools


def read_input(fn):
    with open(fn) as f:
        inp = [i.strip('\n') for i in f.readlines()]
        inp = [(i, j, 0) for j, jv in enumerate(inp) for i, iv in enumerate(jv)
               if iv == '#']
        return inp


def super_looper(cubes, maxDim, dimension=1):
    # only loop over relevant coordinates
    minN = min([x[dimension-1] for x in cubes]) - 1
    maxN = max([x[dimension-1] for x in cubes]) + 2
    if dimension == maxDim:
        for i in range(minN, maxN):
            yield (i,)
    else:
        for i in range(minN, maxN):
            for j in super_looper(cubes, max_dimension, dimension+1):
                yield (i,) + j


def solve1(inp):
    cubes = set(inp)
    for _ in range(6):
        new = set()
        for x, y, z in super_looper(cubes, 3):
            neighbours = 0
            for dx, dy, dz in itertools.product([-1, 0, 1], repeat=3):
                if dx != 0 or dy != 0 or dz != 0:
                    if (x + dx, y + dy, z + dz) in cubes:
                        neighbours += 1
            if (x, y, z) in cubes:
                if neighbours in [2, 3]:
                    new.add((x, y, z))
            else:
                if neighbours == 3:
                    new.add((x, y, z))
        cubes = new
    print(len(cubes))


def solve2(inp):
    cubes = set([(*i, 0) for i in inp])
    for _ in range(6):
        new = set()
        for x, y, z, w in super_looper(cubes, 4):
            neighbours = 0
            for dx, dy, dz, dw in itertools.product([-1, 0, 1], repeat=4):
                if dx != 0 or dy != 0 or dz != 0 or dw != 0:
                    if (x + dx, y + dy, z + dz, w + dw) in cubes:
                        neighbours += 1
            if (x, y, z, w) in cubes:
                if neighbours in [2, 3]:
                    new.add((x, y, z, w))
            else:
                if neighbours == 3:
                    new.add((x, y, z, w))
        cubes = new
    print(len(cubes))


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
