use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::iter::zip;

use rayon::{iter::ParallelIterator, str::ParallelString};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn arrangements(springs: &[char], target_groups: &[usize], total: u32, bar: &ProgressBar) -> u32 {
    let current_groups = springs
        .split(|c| *c != '#')
        .filter_map(|group| {
            if group.is_empty() {
                None
            } else {
                Some(group.len())
            }
        })
        .collect::<Vec<usize>>();

    if current_groups == target_groups {
        bar.inc(1);
        return 1;
    }
    if !springs.contains(&'?') {
        // dbg!(current_groups);
        return 0;
    }
    // max spring group is larger than max group:
    if current_groups.iter().max() > target_groups.iter().max() {
        return 0;
    }

    let mut current_groups = current_groups;
    current_groups.sort();
    current_groups.reverse();
    let mut groups_sorted = target_groups.to_vec();
    groups_sorted.sort();
    groups_sorted.reverse();
    if zip(current_groups.iter(), groups_sorted.iter())
        .map(|(current_size, target_size)| current_size > target_size)
        .any(|b| b)
    {
        return 0;
    }

    // not enough '?' and '#' to fill the groups:
    if current_groups.iter().sum::<usize>() + springs.iter().filter(|&&c| c == '?').count()
        < target_groups.iter().sum::<usize>()
    {
        return 0;
    }

    // check if the prefix without '?' is already larger than the target
    let binding = [&['.'], springs].concat();
    let current_fixed_springs = binding
        .split(|c| *c == '?')
        .find(|group| !group.is_empty())
        .expect("fixed prefix");
    let current_fixed_groups = current_fixed_springs
        .split(|c| *c == '.')
        .filter_map(|group| {
            if group.is_empty() {
                None
            } else {
                Some(group.len())
            }
        })
        .collect::<Vec<usize>>();
    if zip(current_fixed_groups.iter(), target_groups.iter())
        .map(|(current_size, target_size)| current_size > target_size)
        .any(|b| b)
    {
        return 0;
    }
    // replace the first '?' with either '.' or '#'
    let first_question_mark = springs.iter().position(|&c| c == '?').expect("?");

    let springs_dot = {
        let mut springs_dot = springs.to_vec();
        springs_dot[first_question_mark] = '.';
        springs_dot
    };

    let springs_hash = {
        let mut springs_hash = springs.to_vec();
        springs_hash[first_question_mark] = '#';
        springs_hash
    };

    total
        + arrangements(&springs_hash, target_groups, total, bar)
        + arrangements(&springs_dot, target_groups, total, bar)
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, groups) = line.split_once(' ').expect("<springs> <groups>");
    // let springs: Vec<char> = springs.chars().collect();
    let groups: Vec<usize> = groups
        .split(',')
        .map(|s| s.parse().expect("numbers"))
        .collect();

    (
        [springs].repeat(5).join("?").chars().collect(),
        groups.repeat(5),
    )
}

fn part2(input: &str) -> u32 {
    let multi_bar = MultiProgress::new();

    let num_lines = input.lines().count();

    let total_bar = ProgressBar::new(num_lines as u64);
    let total_bar = multi_bar.add(total_bar);

    input
        .par_lines()
        .map(|line| {
            let bar = ProgressBar::new(0);

            let style = ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{human_pos}] - {per_sec}")
                .expect("valid style");

            bar.set_style(style);
            let bar = multi_bar.add(bar);

            // new progress bars won't show unless they tick() first, for some reason...
            bar.tick();

            let (springs, groups) = parse_line(line);
            let result = arrangements(&springs, &groups, 0, &bar);
            bar.println(format!("{}: {}", line, result));

            total_bar.inc(1);
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // let lines: Vec<(Vec<char>, Vec<usize>)> = include_str!("example.txt")
        //     .lines()
        //     .map(parse_line)
        //     .collect();

        // assert_eq!(arrangements(&lines[0].0, &lines[0].1, 0), 1);
        // assert_eq!(arrangements(&lines[1].0, &lines[1].1, 0), 16384);
        // assert_eq!(arrangements(&lines[2].0, &lines[2].1, 0), 1);
        // assert_eq!(arrangements(&lines[3].0, &lines[3].1, 0), 16);
        // assert_eq!(arrangements(&lines[4].0, &lines[4].1, 0), 2500);
        // assert_eq!(arrangements(&lines[5].0, &lines[5].1, 0), 506250);

        assert_eq!(part2(include_str!("example.txt")), 525152);
    }
}
