fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[allow(non_snake_case)]
fn HASH(s: &str) -> u32 {
    let mut current_value = 0;

    s.chars().for_each(|c| {
        dbg!(c);
        dbg!(c as u32);
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
        dbg!(current_value);
    });
    current_value
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .next()
        .expect("first line")
        .split(',')
        .map(HASH)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 1320);
    }
}
