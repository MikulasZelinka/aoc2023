use std::{collections::HashSet, ops::ControlFlow};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut cards: Vec<u32> = vec![1; 214];
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (winning, my) = line.split_once('|').expect(" | delimited cards");
            let winning: HashSet<u32> = winning
                .split_once(':')
                .expect("Card <id>: <numbers>")
                .1
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("number"))
                .collect();

            let copies_of_this_card = cards[i];
            let mut num_winning = 0;
            my.split_whitespace()
                .map(|n| n.parse::<u32>().expect("number"))
                .try_for_each(|n| {
                    if winning.contains(&n) {
                        num_winning += 1;

                        let index_of_next = i + num_winning;
                        if index_of_next >= cards.len() {
                            return ControlFlow::Break(n);
                        }

                        cards[i + num_winning] += copies_of_this_card;
                    }
                    ControlFlow::Continue(())
                });

            cards[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 30);
    }
}
