#!/usr/bin/env python3

import re
from collections import Counter
from functools import reduce


def read_input(fn):
    with open(fn) as f:
        return [i.strip('\n').split(' bags contain') for i in f.readlines()]


def parse(inp):
    r = re.compile('(\d*) (\w* \w*) bag')
    return {k: {i[1]: i[0] for i in r.findall(v)} for k, v in inp}


def find_path(mybag, bag, inp):
    if 'no other' in inp[bag]:
        return []
    if mybag in inp[bag]:
        return [bag]
    valid = [j for i in inp[bag] for j in find_path(mybag, i, inp)]
    if valid:
        return [bag] + valid
    return []


def addup(bag, inp, times=1):
    if 'no other' in inp[bag]:
        return Counter({bag: times})
    bags = reduce(lambda a, b: a + b,
                  [addup(i, inp,
                         int(t) * times) for i, t in inp[bag].items()])
    return Counter({bag: times}) + bags


def solve1(inp):
    valid = set()
    mybag = 'shiny gold'
    for bag in inp:
        valid.update(set(find_path(mybag, bag, inp)))
    return len(valid)


def solve2(inp):
    mybag = 'shiny gold'
    shopping_cart = addup(mybag, inp)
    return sum(shopping_cart.values()) - 1  # do not count shiny gold bag


known1 = {'test': 4}
for fn in ['test', 'input']:
    inp = parse(read_input(fn))
    result = solve1(inp)
    if fn in known1:
        assert known1[fn] == result
    print(f'{fn}: part1: {result}')

known2 = {'test': 32, 'test2': 126}
for fn in ['test', 'test2', 'input']:
    inp = parse(read_input(fn))
    result = solve2(inp)
    if fn in known2:
        assert known2[fn] == result
    print(f'{fn}: part2: {result}')
