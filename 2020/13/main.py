#!/usr/bin/env python3


def read_input(fn):
    with open(fn) as f:
        inp = [i.strip('\n') for i in f.readlines()]
        timestamp = int(inp[0])
        offsets = {
            int(i): o
            for o, i in enumerate(inp[1].split(',')) if i != 'x'
        }
        return timestamp, offsets


def solve1(ts, offsets):
    wait_times = []
    for ID in offsets:
        wait_time = ID - ts % ID
        wait_times.append((wait_time, ID, wait_time * ID))
    wait_times.sort(key=lambda x: x[0])
    print(wait_times[0][2])


def solve2(ts, offsets):
    # chinese remainder theorem
    t = 0
    delta = 1
    for ID, offset in offsets.items():
        while (t + offset) % ID != 0:
            t += delta
        delta *= ID
    print(t)


for fn in ['test', 'input']:
    print('\n' + fn)
    ts, offsets = read_input(fn)
    solve1(ts, offsets)
    solve2(ts, offsets)
