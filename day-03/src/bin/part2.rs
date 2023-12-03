use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Number {
    value: u32,
    line_number: usize,
    start: usize,
    end: usize,
    id: usize,
}

struct Symbol {
    row: usize,
    column: usize,
}

fn part1(input: &str) -> u32 {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut id = 0;

    let regex_number = Regex::new(r"\d+").expect("Valid regex");

    let mut position_to_number: HashMap<(usize, usize), Number> = HashMap::new();

    // using regex, for each line, extract all integers and the indices they start and end at
    // also, remember the line number for each integer
    input.lines().enumerate().for_each(|(line_number, line)| {
        // populate symbols
        line.chars().enumerate().for_each(|(column, character)| {
            if character == '*' && !character.is_numeric() {
                symbols.push(Symbol {
                    row: line_number,
                    column,
                });
            }
        });

        // populate numbers
        regex_number.find_iter(line).for_each(|m| {
            let number = Number {
                value: m.as_str().parse::<u32>().expect("Valid number from regex"),
                line_number,
                start: m.start(),
                end: m.end(),
                id,
            };

            numbers.push(number);
            id += 1;

            // @TODO: what's the best way to do this, perhaps without (implicit) cloning?
            for i in number.start..number.end {
                position_to_number.insert((number.line_number, i), number);
            }
        });
    });

    symbols
        .iter()
        .filter_map(|symbol| {
            let mut numbers = HashSet::new();

            for i in -1..=1 {
                for j in -1..=1 {
                    let row = symbol.row as i32 + i;
                    let column = symbol.column as i32 + j;
                    if row < 0 || column < 0 {
                        continue;
                    }
                    if let Some(number) = position_to_number.get(&(row as usize, column as usize)) {
                        numbers.insert(number);
                    }
                }
            }

            if numbers.len() == 2 {
                Some(numbers.iter().map(|number| number.value).product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 467835);
    }
}
