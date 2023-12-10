from pathlib import Path

example = open(Path(__file__).parent / "example.txt").read()

input = open(Path(__file__).parent / "input.txt").read()


def parse(input_string):
    return [[int(x) for x in line.split()] for line in input_string.splitlines()]


def part1(seqs):
    def next_number(seq):
        if all([x == 0 for x in seq]):
            return 0
        return seq[-1] + next_number([seq[i + 1] - seq[i] for i in range(len(seq) - 1)])

    return sum(next_number(seq) for seq in seqs)


assert part1(parse(example)) == 114
print(part1(parse(input)))


def part2(seqs):
    return part1([list(reversed(seq)) for seq in seqs])


assert part2(parse(example)) == 2
print(part2(parse(input)))
