from pathlib import Path

import numpy as np
from tqdm import tqdm

example = open(Path(__file__).parent / "example.txt").read()

input = open(Path(__file__).parent / "input.txt").read()


def parse(input_string):
    return np.array([list(x) for x in input_string.split()])


def cycle(platform):
    for _ in range(4):
        platform = slide_up(platform)
        platform = np.rot90(platform, k=-1)

    return platform


def load(platform):
    return np.sum((platform == "O") * np.arange(platform.shape[0], 0, -1)[:, None])


def slide_up(platform):
    num_rows, num_cols = platform.shape

    platform_after_slide = platform.copy()

    for col in range(num_cols):
        top_row = 0
        for row in range(num_rows):
            if platform[row, col] == "O":
                platform_after_slide[row, col] = "."
                platform_after_slide[top_row, col] = "O"

                top_row += 1
            elif platform[row, col] == "#":
                top_row = row + 1

    return platform_after_slide


def part1(platform):
    return load(slide_up(platform))


def part2(platform, num_cycles=1_000_000_000):
    platform_to_i = {}
    i_to_platform = {}

    for i in tqdm(range(1, num_cycles + 1)):
        platform = cycle(platform)

        platform_bytes = platform.tobytes()

        if (p_i := platform_to_i.get(platform_bytes, None)) is not None:
            cycle_length = i - p_i
            return load(
                np.frombuffer(
                    i_to_platform[p_i + ((num_cycles - i) % cycle_length)], dtype="<U1"
                ).reshape(platform.shape)
            )

        platform_to_i[platform_bytes] = i
        i_to_platform[i] = platform_bytes

    return load(platform)


assert part1(parse(example)) == 136
print(part1(parse(input)))


assert part2(parse(example)) == 64
print(part2(parse(input)))
