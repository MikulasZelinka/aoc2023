"""
Python because this was done while teaching a Python class.

This most naive solution unfortunately still finishes in under two minutes for part 2.
"""

from pathlib import Path

import numpy as np
from tqdm import tqdm


def compute_distance(T, hold_time):
    return (T - hold_time) * hold_time


def solve(input_file_path, part="part1"):
    print(input_file_path, part)

    with open(Path(__file__).parent / input_file_path) as f:
        if part == "part1":
            times = [int(x) for x in f.readline().split()[1:]]
            records = [int(x) for x in f.readline().split()[1:]]
        elif part == "part2":
            times = [int("".join(f.readline().split()[1:]).replace(" ", ""))]
            records = [int("".join(f.readline().split()[1:]).replace(" ", ""))]

    num_ways_to_beat_record = []

    for T, record in tqdm(zip(times, records)):
        num_better_than_record = 0
        for hold_time in tqdm(range(T + 1)):
            d = compute_distance(T, hold_time)
            if d > record:
                num_better_than_record += 1

        num_ways_to_beat_record.append(num_better_than_record)

    print(times)
    print(records)
    print(np.prod(num_ways_to_beat_record))
    print()


solve("example.txt")
solve("input.txt")

solve("example.txt", part="part2")
solve("input.txt", part="part2")
