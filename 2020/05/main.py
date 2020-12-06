#!/usr/bin/env python


def read_input(fn):
    with open(fn) as f:
        seats = [i for i in f.read().split('\n') if len(i) > 0]
        return seats


def part(x):
    middle = (x[0] + x[1]) // 2
    return (x[0], middle), (middle, x[1])


def calc_seatID(row, column):
    return 8 * row + column


def seat(s):
    rows = (0, 127)
    columns = (0, 7)
    for i in range(7):
        front, back = part(rows)
        if s[i] == 'F':
            rows = front
        elif s[i] == 'B':
            rows = back
        else:
            print('wrong input')
            exit(1)
    for i in range(7, 10):
        front, back = part(columns)
        if s[i] == 'L':
            columns = front
        elif s[i] == 'R':
            columns = back
        else:
            print('wrong input')
            exit(1)
    row = rows[1]
    column = columns[1]
    seatID = calc_seatID(row, column)
    return seatID


def solve1(fn):
    print(fn)
    print('part1:')
    inp = read_input(fn)
    ids = []
    for i in inp:
        ids.append(seat(i))
    print(f'max seatID: {max(ids)}')
    return ids


def solve2(ids):
    print('part2:')
    ids.sort()
    diffs = [ids[i + 1] - ids[i] for i in range(len(ids) - 1)]

    if 2 not in diffs:
        print('could not find seat')
        return

    my_seat = ids[diffs.index(2)] + 1
    print(f'my seatID: {my_seat}')


for fn in ['test', 'input']:
    ids = solve1(fn)
    solve2(ids)
    print('\n')
