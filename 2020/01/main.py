#!/usr/bin/env python


def solve2(inp):
    for i, j in enumerate(inp):
        for k in inp[1+i:]:
            if (j + k) == 2020:
                print(j * k)

def solve3(inp):
    for i in range(len(inp)):
        for j in range(1 + i, len(inp)):
            for k in range(1 + j, len(inp)):
                m, n, o = inp[i], inp[j], inp[k]
                if (m + n + o) == 2020:
                    print(m * n * o)


# fn = 'test'
fn = 'input'

with open(fn) as f:
    inp = [int(i) for i in f.read().split('\n') if len(i) > 0]

solve2(inp)
solve3(inp)
