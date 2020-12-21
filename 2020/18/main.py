#!/usr/bin/env python3


def read_input(fn):
    with open(fn) as f:
        inp = [i.strip('\n') for i in f.readlines()]
        inp = [i.replace(' ', '') for i in inp]
        return inp


class syntax_tree():
    def __init__(self, operator=None, L=None, R=None):
        self.operator = operator
        self.L = L
        self.R = R

    def __repr__(self):
        return self.traverse()

    def traverse(self, i=0):
        s = i * '\t' + self.operator + '\n'
        if self.L:
            s += self.L.traverse(i + 1)
        if self.R:
            s += self.R.traverse(i + 1)
        return s

    def eval(self):
        if not self.L and not self.R:
            return int(self.operator)
        L = self.L.eval()
        R = self.R.eval()
        if self.operator == '+':
            return L + R
        elif self.operator == '*':
            return L * R
        else:
            raise ValueError('wrong operator {self.operator}')


def tokenize(s):
    level = 0
    operators = ['*', '+']
    ops = []
    for i, s in enumerate(s):
        if s == '(':
            level += 1
        if s == ')':
            level -= 1
        if level == 0 and s in operators:
            ops.append(i)
        if len(ops) == 2:
            break
    if len(ops) == 0:
        return s
    if len(ops) == 1:
        return s[0:ops[0]], s[ops[0]], s[ops[0] + 1:]
    if len(ops) == 2:
        return (s[0:ops[0]], s[ops[0]], s[ops[0] + 1:ops[1]], s[ops[1]],
                s[ops[1] + 1:])


def parse(syntax, t=None):
    tokens = tokenize(syntax)
    if isinstance(tokens, str):
        if tokens == '':
            return t
        if tokens[0] == '(':
            return parse(tokens[1:-1])
        return syntax_tree(tokens, t)

    first, op, second, *tokens = tokens
    if t is None:
        t = syntax_tree(op, parse(first), parse(second))
    else:
        t = syntax_tree(op, t, parse(second))
    return parse(''.join(tokens), t)


def add_before_mult(s, i=0):  # add parentheses around all sums
    buff = []
    last = 0
    insum = False
    while i < len(s) and s[i] != ')':
        last += 1
        if s[i] == '(':
            ks, i = add_before_mult(s, i + 1)
            buff.append(ks)
        else:
            buff.append(s[i])
        if s[i] == '+':
            insum = True
        if i + 1 == len(s) or s[i + 1] == '*' or s[i + 1] == ')':
            if insum:
                if buff[-last] == '*':
                    last -= 1
                parts = buff[-last:]
                buff = buff[:-last]
                buff.append(parts)
            insum = False
            last = 0
        i += 1
    return buff, i


def stringy_thingy(s):
    rv = ''
    for i in s:
        if isinstance(i, str):
            rv += i
        else:
            rv += f'({stringy_thingy(i)})'
    return rv


def solve1(inp):
    s = 0
    for i in inp:
        print(i)
        t = parse(i)
        print(t)
        s += t.eval()
    print(s)


def solve2(inp):
    s = 0
    for i in inp:
        print(i)
        k, _ = add_before_mult(i)
        t = parse(stringy_thingy(k))
        print(t)
        r = t.eval()
        s += r
    print(s)


for fn in ['test', 'input']:
    print('\n' + fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
