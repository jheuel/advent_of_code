#!/usr/bin/env python3

import re

re_id = re.compile('Tile (\d+):')


def read_input(fn):
    with open(fn) as f:
        inp = f.read().split('\n\n')
        inp = [(re_id.match(tile[0]).group(1), tile[1:])
                for tile in [i.split('\n') for i in inp]]
        return inp


def solve1(inp):
    print(inp)


for fn in ['test']:
    print('\n' + fn)
    inp = read_input(fn)

    solve1(inp)
