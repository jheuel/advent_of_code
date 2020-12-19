#!/usr/bin/env python3


def read_input(fn):
    with open(fn) as f:
        return [int(i.strip('\n')) for i in f.readlines()]


def sum_of(number, numbers):
    for i, m in enumerate(numbers):
        for n in numbers[i:]:
            if m + n == number:
                return True
    return False


def solve1(inp, preamble):
    for i in range(preamble, len(inp)):
        if not sum_of(inp[i], inp[i - preamble:i]):
            return inp[i]


def solve2(inp, number):
    correct_sum = []
    series = []
    for i in range(len(inp)):
        for j in inp[i:]:
            series.append(j)
            if sum(series) >= number:
                if sum(series) == number:
                    correct_sum.append(series)
                series = []
                break
    correct_sum.sort(key=len)
    return min(correct_sum[-1]), max(correct_sum[-1])


preamble = {
    'test': 5,
    'input': 25,
}

for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    n = solve1(inp, preamble[fn])
    a, b = solve2(inp, n)

    print(f'part1: {n}')
    print(f'longest sum is between {a} and {b}, which sum up to {a+b}')
