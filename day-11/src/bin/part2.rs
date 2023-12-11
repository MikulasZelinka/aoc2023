mod part1;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input, 1_000_000 - 1);
    dbg!(output);
}

fn part2(input: &str, expansion: usize) -> usize {
    part1::part1(input, expansion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt"), 1), 374);
        assert_eq!(part2(include_str!("example.txt"), 10 - 1), 1030);
        assert_eq!(part2(include_str!("example.txt"), 100 - 1), 8410);
    }
}
