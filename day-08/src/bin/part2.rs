use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().expect("RL directions");

    assert!(lines.next().expect("second (empty) line").is_empty());

    let mut network = HashMap::new();
    lines.for_each(|line| {
        network.insert(&line[0..3], (&line[7..10], &line[12..15]));
    });

    dbg!(&instructions);
    dbg!(&network);

    let mut current_node = "AAA";

    for (i, instruction) in instructions.chars().cycle().enumerate() {
        let (left, right) = network.get(current_node).expect("current node exists");
        let next_node = match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("unknown instruction: {}", instruction),
        };
        if next_node == &"ZZZ" {
            return i + 1;
        }
        dbg!(i + 1, instruction, current_node, next_node);
        current_node = next_node;
    }

    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example1.txt")), 2);
        assert_eq!(part2(include_str!("example2.txt")), 6);
    }
}
