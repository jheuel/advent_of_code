#!/usr/bin/env python3


def read_input(fn):
    with open(fn) as f:
        inp = [list(i.strip('\n')) for i in f.readlines()]
        return inp


def flatten(l):
    return [i for s in l for i in s]


def copy(l):
    return [i[:] for i in l]


def ray(seats, i, j, direction):
    while True:
        i += direction[0]
        j += direction[1]
        if i == len(seats): break
        if j == len(seats[0]): break
        if i < 0: break
        if j < 0: break
        if seats[i][j] == 'L':
            return 0
        if seats[i][j] == '#':
            return 1
    return 0


def next_neighbour_policy(seats, i, j):
    block = [
        b[max(0, j - 1):min(len(seats[0]), j + 2)]
        for b in seats[max(0, i - 1):min(len(seats), i + 2)]
    ]
    occupied = flatten(block).count('#')
    if seats[i][j] == 'L':
        if occupied == 0:
            return '#'
    elif seats[i][j] == '#':
        if occupied >= 5:  # counts center "#"
            return 'L'
    return seats[i][j]


def next_visible_policy(seats, i, j):
    directions = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ]
    occupied = sum([ray(seats, i, j, d) for d in directions])

    if seats[i][j] == 'L':
        if occupied == 0:
            return '#'
    elif seats[i][j] == '#':
        if occupied >= 5:
            return 'L'
    return seats[i][j]


def run(inp, policy):
    seats = copy(inp)
    new_seats = None
    while seats != new_seats:
        if new_seats:
            seats = copy(new_seats)
        else:
            new_seats = copy(seats)

        for i in range(len(seats)):
            for j in range(len(seats[0])):
                new_seats[i][j] = policy(seats, i, j)

    print(flatten(seats).count('#'))


def solve1(inp):
    run(inp, next_neighbour_policy)


def solve2(inp):
    run(inp, next_visible_policy)


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
