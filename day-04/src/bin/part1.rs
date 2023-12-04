use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, my) = line.split_once('|').expect(" | delimited cards");
            let winning: HashSet<u32> = winning
                .split_once(':')
                .expect("Card <id>: <numbers>")
                .1
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("number"))
                .collect();
            let mut points: u32 = 0;

            my.split_whitespace()
                .map(|n| n.parse::<u32>().expect("number"))
                .for_each(|n| {
                    if winning.contains(&n) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    }
                });

            points
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 13);
    }
}
