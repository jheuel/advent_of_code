#!/usr/bin/env python


def read_input(fn):
    with open(fn) as f:
        inp = [i.strip('\n').split(' ') for i in f.readlines()]
        return [[i[0], int(i[1])] for i in inp]


def solve1(inp):
    _, accumulator = run(inp)
    print(f'accumulator: {accumulator}')


def run(inp):
    seen = set()
    accumulator = 0
    current = 0
    while current < len(inp):
        if current in seen:
            return False, accumulator
        cmd, val = inp[current]
        if cmd == 'nop':
            pass
        elif cmd == 'acc':
            accumulator += val
        elif cmd == 'jmp':
            current += val - 1
        seen.add(current)
        current += 1
    return True, accumulator


def solve2(inp):
    jmps = [(i, 'jmp') for i, x in enumerate(inp) if x[0] == 'jmp']
    nops = [(i, 'nop') for i, x in enumerate(inp) if x[0] == 'nop']
    changes = jmps + nops
    change = {'jmp': 'nop', 'nop': 'jmp'}

    for position, instruction in changes:
        modified_input = [i[:] for i in inp]
        modified_input[position][0] = change[instruction]
        fixed, accumulator = run(modified_input)
        if fixed:
            print(f'accumulator: {accumulator}')
            return


for fn in ['test', 'input']:
    print(fn)
    inp = read_input(fn)
    solve1(inp)
    solve2(inp)
