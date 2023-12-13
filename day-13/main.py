from pathlib import Path

import numpy as np

example = open(Path(__file__).parent / "example.txt").read()

input = open(Path(__file__).parent / "input.txt").read()


def parse(input_string):
    patterns = []
    for pattern in input_string.split("\n\n"):
        patterns.append(np.array([list(x) for x in pattern.split()]))
    # print(patterns)
    return patterns


def mirror_value(pattern: np.array, target_diff=0):
    for middle in range(1, pattern.shape[0]):
        valid_range = min(middle, pattern.shape[0] - middle)
        if (
            np.sum(
                np.not_equal(
                    pattern[middle - valid_range : middle, :],
                    pattern[middle : middle + valid_range, :][::-1],
                )
            )
            == target_diff
        ):
            return middle * 100

    for middle in range(1, pattern.shape[1]):
        valid_range = min(middle, pattern.shape[1] - middle)
        if (
            np.sum(
                np.not_equal(
                    pattern[:, middle - valid_range : middle],
                    np.fliplr(pattern[:, middle : middle + valid_range]),
                )
            )
            == target_diff
        ):
            return middle

    print()
    print("\n".join("".join(x) for x in pattern))
    print()
    print("no mirror found")
    raise ValueError("No mirror found")


def part1(patterns: list[np.array]):
    return sum(mirror_value(pattern, target_diff=0) for pattern in patterns)


def part2(patterns: list[np.array]):
    return sum(mirror_value(pattern, target_diff=1) for pattern in patterns)


assert part1(parse(example)) == 405
print(part1(parse(input)))

assert part2(parse(example)) == 400
print(part2(parse(input)))
