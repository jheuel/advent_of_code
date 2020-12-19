#!/usr/bin/env python3

from collections import defaultdict
import re
scheme_re = re.compile('(^.*): (.*)-(.*) or (.*)-(.*)$')


def read_input(fn):
    with open(fn) as f:
        scheme, my_ticket, nearby_tickets = [
            i.split('\n') for i in f.read().split('\n\n')]
        scheme = [i.group(1, 2, 3, 4, 5)
                  for i in [scheme_re.match(s) for s in scheme]]
        my_ticket = my_ticket[1].split(',')
        nearby_tickets = [i.split(',') for i in nearby_tickets[1:]]
        scheme = [(i[0], [int(j) for j in i[1:]]) for i in scheme]
        my_ticket = [int(i) for i in my_ticket]
        nearby_tickets = [[int(j) for j in i] for i in nearby_tickets]

        return scheme, my_ticket, nearby_tickets


def fits_scheme(s, n):
    return s[0] <= n and n <= s[1] or s[2] <= n and n <= s[3]


def invalid(scheme, number):
    is_invalid = True
    for _, s in scheme:
        if fits_scheme(s, number):
            is_invalid = False
    return is_invalid


def solve1(scheme, nearby_tickets):
    s = 0
    to_delete = []
    for i, ticket in enumerate(nearby_tickets):
        for number in ticket:
            if invalid(scheme, number):
                s += number
                to_delete.append(i)
    print(s)
    for i in to_delete[::-1]:
        nearby_tickets.pop(i)
    return nearby_tickets


def solve2(scheme, my_ticket, nearby_tickets):
    works = defaultdict(dict)
    for i in range(len(nearby_tickets[0])):
        for j in range(len(scheme)):
            works[i][j] = True
    for ticket in nearby_tickets:
        for column, number in enumerate(ticket):
            for i, s in enumerate(scheme):
                if not fits_scheme(s[1], number):
                    works[column][i] = False

    possible = [(i, [k for k, v in j.items() if v]) for i, j in works.items()]
    possible.sort(key=lambda x: len(x[1]))

    mapping = {}
    for f, p in possible:
        for i in p:
            if i in mapping:
                continue
            mapping[i] = f

    fields = [i for i, s in enumerate(scheme) if 'departure' in s[0]]
    product = 1
    for f in fields:
        product *= my_ticket[mapping[f]]
    print(product)


for fn in ['test', 'test2', 'input']:
    print('\n' + fn)
    scheme, my_ticket, nearby_tickets = read_input(fn)
    nearby_tickets = solve1(scheme, nearby_tickets)
    solve2(scheme, my_ticket, nearby_tickets)
