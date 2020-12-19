#!/usr/bin/env python3

import re


required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

value_ranges = {
    'byr': [1920, 2002],
    'iyr': [2010, 2020],
    'eyr': [2020, 2030],
}

height_ranges = {
    'cm': [150, 193],
    'in': [59, 76],
}

eye_colors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']


def read_input(fn):
    with open(fn) as f:
        passports = [i for i in f.read().split('\n\n') if len(i) > 0]
        passports = [i.replace('\n', ' ').strip() for i in passports]
        passports = [i.split(' ') for i in passports]
        passports = [{k: v for k, v in [j.split(':') for j in i]}
                        for i in passports]
        return passports


def solve1(fn):
    print(f'part1: {fn}', end=': ')
    inp = read_input(fn)

    valid = 0
    for i in inp:
        if all([r in i for r in required_fields]):
            valid += 1
    print(f'valid passports: {valid}')


def solve2(fn):
    print(f'part2: {fn}', end=': ')
    inp = read_input(fn)

    valid = 0
    for passport in inp:
        if check(passport):
            valid += 1

    print(f'valid passports: {valid}')


def isNumber(s):
    return re.match('^\d*$', s)


def isColor(s):
    return re.match('^#[0-9a-f]*$', s)


def isPassport(s):
    return re.match('^\d{9}$', s)


def check(passport):
    if not all([r in passport for r in required_fields]):
        return False
    for field, value_range in value_ranges.items():
        value = int(passport[field])
        if value < value_range[0] or value > value_range[1]:
            return False
    if not isNumber(passport['hgt'][:-2]):
        return False
    if not passport['hgt'][-2:] in height_ranges:
        return False
    for unit, value_range in height_ranges.items():
        if unit in passport['hgt']:
            value = int(passport['hgt'][:-2])
            if value < value_range[0] or value > value_range[1]:
                return False
    if not isColor(passport['hcl']):
        return False
    if not passport['ecl'] in eye_colors:
        return False
    if not isPassport(passport['pid']):
        return False
    return True


for fn in ['test', 'input']:
    solve1(fn)
    solve2(fn)
