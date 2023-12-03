use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Number {
    value: u32,
    line_number: usize,
    start: usize,
    end: usize,
}

struct Symbol {
    row: usize,
    column: usize,
}

fn part1(input: &str) -> u32 {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let regex_number = Regex::new(r"\d+").expect("Valid regex");

    // using regex, for each line, extract all integers and the indices they start and end at
    // also, remember the line number for each integer
    input.lines().enumerate().for_each(|(line_number, line)| {
        // populate symbols
        line.chars().enumerate().for_each(|(column, character)| {
            if character != '.' && !character.is_numeric() {
                symbols.push(Symbol {
                    row: line_number,
                    column,
                });
            }
        });

        // populate numbers
        regex_number.find_iter(line).for_each(|m| {
            numbers.push(Number {
                value: m.as_str().parse::<u32>().expect("Valid number from regex"),
                line_number,
                start: m.start(),
                end: m.end(),
            });
        });
    });

    let mut covered_indices: HashSet<(usize, usize)> = HashSet::new();

    // for each symbol, add the indices around it in 2D (8 directions):
    symbols.iter().for_each(|symbol| {
        for i in -1..=1 {
            for j in -1..=1 {
                let row = symbol.row as i32 + i;
                let column = symbol.column as i32 + j;
                if row >= 0 && column >= 0 {
                    covered_indices.insert((row as usize, column as usize));
                }
            }
        }
    });

    //  symbols.iter().for_each(|symbol| {

    // }

    // numbers.iter().map(|number| number.value).sum()
    numbers
        .iter()
        .filter_map(|number| {
            // if any of the number positions is covered, return the number, else None
            if (number.start..number.end)
                .any(|i| covered_indices.contains(&(number.line_number, i)))
            {
                Some(number.value)
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
        assert_eq!(part1(include_str!("example.txt")), 4361);
    }
}
