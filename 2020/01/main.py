!/usr/bin/env python


def read_input(fn):
    with open(fn) as f:
        return [int(i) for i in f.read().split('\n') if len(i) > 0]


def solve2(inp):
    for i, j in enumerate(inp):
        for k in inp[1 + i:]:
            if (j + k) == 2020:
                print(j * k)


def solve3(inp):
    for i, m in enumerate(inp):
        for j, n in enumerate(inp[1 + i:]):
            for o in inp[1 + i + j:]:
                if (m + n + o) == 2020:
                    print(m * n * o)


for fn in ['test', 'input']:
    print(fn)
    inp = read_input(fn)

    solve2(inp)
    solve3(inp)
    print()
