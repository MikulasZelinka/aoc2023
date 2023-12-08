use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().expect("RL directions");

    assert!(lines.next().expect("second (empty) line").is_empty());

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_nodes: Vec<&str> = vec![];
    lines.for_each(|line| {
        let key = &line[0..3];
        network.insert(key, (&line[7..10], &line[12..15]));

        if key.ends_with('A') {
            current_nodes.push(key);
        }
    });

    dbg!(&instructions);
    dbg!(&network);

    let bar = ProgressBar::new(0);

    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{human_pos}] - {per_sec}")
        .expect("valid style");
    bar.set_style(style);

    for (i, instruction) in instructions.chars().cycle().enumerate() {
        let mut next_nodes: Vec<&str> = vec![];

        for current_node in current_nodes {
            let (left, right) = network.get(current_node).expect("current node exists");
            let next_node = match instruction {
                'L' => left,
                'R' => right,
                _ => panic!("unknown instruction: {}", instruction),
            };

            // dbg!(i + 1, instruction, current_node, next_node);
            next_nodes.push(next_node);
        }

        if next_nodes.iter().all(|node| node.ends_with('Z')) {
            return i + 1;
        }

        current_nodes = next_nodes;
        bar.inc(1);
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
        assert_eq!(part2(include_str!("example3.txt")), 6);
    }
}
