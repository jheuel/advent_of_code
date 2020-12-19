#!/usr/bin/env python3

from math import sin, cos, pi


def read_input(fn):
    with open(fn) as f:
        inp = [i.strip('\n') for i in f.readlines()]
        inp = [(i[0], int(i[1:])) for i in inp]
        return inp


def rad(d):
    return d / 180 * pi


class BaseShip:
    def __init__(self):
        self.x = 0
        self.y = 0

    def direction(self, op):
        d = {
            'N': 90,
            'E': 0,
            'W': 180,
            'S': 270,
        }
        if op in d:
            return d[op]
        else:
            raise KeyError(f'invalid direction {op}')

    def run(self, instruction):
        op, val = instruction
        if op == 'L':
            self.turn(val)
        if op == 'R':
            self.turn(-val)
        if op in ['N', 'W', 'S', 'E', 'F']:
            self.move(val, op)


class Ship(BaseShip):
    def __init__(self):
        super().__init__()
        self.pov = 0

    def __str__(self):
        return (f'x: {self.x:6.0f}, '
                f'y: {self.y:6.0f}, '
                f'pov: {self.pov:4.0f}, '
                f'd: {abs(self.x) + abs(self.y):.0f}')

    def move(self, val, op):
        d = self.direction(op) if op != 'F' else self.pov
        v = (cos(rad(d)), sin(rad(d)))
        self.x += v[0] * val
        self.y += v[1] * val

    def turn(self, val):
        self.pov = (self.pov + val) % 360


class Ship2(BaseShip):
    def __init__(self):
        super().__init__()
        self.wp_x = 10
        self.wp_y = 1

    def __str__(self):
        return (f'x: {self.x:6.0f}, '
                f'y: {self.y:6.0f}, '
                f'wp_x: {self.wp_x:4.0f}, '
                f'wp_y: {self.wp_y:4.0f}, '
                f'd: {abs(self.x) + abs(self.y):.0f}')

    def move(self, val, op):
        if op == 'F':
            self.x += self.wp_x * val
            self.y += self.wp_y * val
            return
        d = self.direction(op)
        v = (cos(rad(d)), sin(rad(d)))
        self.wp_x += cos(rad(d)) * val
        self.wp_y += sin(rad(d)) * val

    def turn(self, val):
        theta = rad(val)
        wp_x = self.wp_x * cos(theta) - self.wp_y * sin(theta)
        wp_y = self.wp_x * sin(theta) + self.wp_y * cos(theta)
        self.wp_x = wp_x
        self.wp_y = wp_y


def solve1(inp):
    s = Ship()
    for i in inp:
        s.run(i)
    print(s)


def solve2(inp):
    s = Ship2()
    for i in inp:
        s.run(i)
    print(s)


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
