#!/usr/bin/env python3

import re

re_mask = re.compile('^mask = (.*)$')
re_mem = re.compile('^mem\[(.*)\] = (.*)$')


def read_input(fn):
    with open(fn) as f:
        return f.read().splitlines()


def run(inp, apply_mask):
    memory = {}
    for i in inp:
        if match := re_mask.match(i):
            mask = match.group(1)
        elif match:= re_mem.match(i):
            address, value = match.group(1, 2)
            apply_mask(mask, int(value), int(address), memory)
    return memory


def expand(n):
    mask = [i == 'X' for i in n]
    for i in range(2**sum(mask)):
        j = len(n)
        replacement = list(f'{i:0{len(mask)}b}')
        while j > 0:
            j -= 1
            if mask[j]:
                n = n[:j] + replacement.pop(-1) + n[j + 1:]
        yield int(n, 2)


def apply_mask_v1(mask, value, address, memory):
    val_binary = f'{value:036b}'
    masked_value = ''.join(
        [v if mask[i] == 'X' else mask[i] for i, v in enumerate(val_binary)])
    memory[address] = int(masked_value, 2)


def apply_mask_v2(mask, value, address, memory):
    addr_binary = f'{address:036b}'
    masked_address = ''.join(
        [a if mask[i] == '0' else mask[i] for i, a in enumerate(addr_binary)])
    for addr in expand(masked_address):
        memory[addr] = value


def solve1(inp):
    memory = run(inp, apply_mask_v1)
    print(sum(memory.values()))


def solve2(inp):
    memory = run(inp, apply_mask_v2)
    print(sum(memory.values()))


for fn in ['test', 'test2', 'input']:
    print('\n' + fn)
    inp = read_input(fn)

    solve1(inp)
    if fn == 'test':
        continue
    solve2(inp)
