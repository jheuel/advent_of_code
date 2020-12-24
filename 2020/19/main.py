#!/usr/bin/env python3

import re

re_or = re.compile('^(.+)\|(.+)$')
re_number = re.compile('(\d+)')
re_match = re.compile('"(.)"')


def read_input(fn):
    with open(fn) as f:
        rules, messages = f.read().split('\n\n')
        rules = {
            k: v
            for k, v in [i.split(': ') for i in rules.strip('\n').split('\n')]
        }
        messages = [i for i in messages.strip('\n').split('\n')]
        return rules, messages


def parse(message, rule, rules):
    if match := re_or.match(rule):
        lhs, rhs = match.group(1, 2)
        return [
            *parse(message, lhs, rules),
            *parse(message, rhs, rules),
        ]
    elif matches := re_number.findall(rule):
        rest = [message]
        for match in matches:
            rest = [i for r in rest for i in parse(r, rules[match], rules)]
        return rest
    elif match := re_match.match(rule):
        word = match.group(1)
        if len(message) == 0 or not message.startswith(word):
            return []
        return [message[1:]]


def solve(rules, messages):
    s = 0
    for message in messages:
        for rest in parse(message, '0', rules):
            if rest == '':
                s += 1
    print(s)


for fn in ['test', 'test2', 'input']:
    print('\n' + fn)
    rules, messages = read_input(fn)
    solve(rules, messages)

    rules['8'] = '42 | 42 8'
    rules['11'] = '42 31 | 42 11 31'
    solve(rules, messages)
