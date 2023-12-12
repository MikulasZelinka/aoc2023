use cached::proc_macro::cached;
use cached::UnboundCache;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

// use rayon::{iter::ParallelIterator, str::ParallelString};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[cached(
    type = "UnboundCache<(Vec<char>, Vec<usize>, usize), usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (springs.clone(), groups.clone(), current_group_len) }"#
)]
fn num_arrangements(
    springs: Vec<char>,
    groups: Vec<usize>,
    current_group_len: usize,
    _bar: &ProgressBar,
) -> usize {
    if springs.is_empty() {
        let result = (groups.is_empty() && current_group_len == 0)
            || (groups.len() == 1 && groups[0] == current_group_len);
        if result {
            _bar.inc(1);
        }
        return result as usize;
    }

    let (current_spring, next_springs) = (springs[0], &springs[1..]);

    match current_spring {
        // '.' means that a group has ended (if it existed)
        '.' => {
            match current_group_len {
                // no group to end, continue
                0 => num_arrangements(next_springs.to_vec(), groups, current_group_len, _bar),
                // successfully completed a group
                n if n == groups[0] => {
                    num_arrangements(springs[1..].to_vec(), groups[1..].to_vec(), 0, _bar)
                }
                // group ended prematurely
                _ => 0,
            }
        }
        // '#' means that a group has started or is continuing
        '#' => {
            if groups.is_empty() {
                // too many groups
                0
            } else {
                num_arrangements(springs[1..].to_vec(), groups, current_group_len + 1, _bar)
            }
        }

        '?' => {
            // recursively try both possibilities by prepending '.' and '#'
            num_arrangements(
                [&['.'], next_springs].concat(),
                groups.clone(),
                current_group_len,
                _bar,
            ) + num_arrangements(
                [&['#'], next_springs].concat(),
                groups,
                current_group_len,
                _bar,
            )
        }

        _ => unreachable!(),
    }
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, groups) = line.split_once(' ').expect("<springs> <groups>");
    let groups: Vec<usize> = groups
        .split(',')
        .map(|s| s.parse().expect("numbers"))
        .collect();

    (
        [springs].repeat(5).join("?").chars().collect(),
        groups.repeat(5),
    )
}

fn part2(input: &str) -> usize {
    let multi_bar = MultiProgress::new();

    let num_lines = input.lines().count();

    let total_bar = ProgressBar::new(num_lines as u64);
    let total_bar = multi_bar.add(total_bar);

    input
        // naivne line-by-line parallelization is slower due to caching
        // .par_lines()
        .lines()
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
            let result = num_arrangements(springs, groups, 0, &bar);
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
        assert_eq!(part2(include_str!("example.txt")), 525152);

        // 1087615125 is too low (was caused by an overflow)
        // assert!(part2(include_str!("input.txt")) > 1087615125);
        assert_eq!(part2(include_str!("input.txt")), 7732028747925);
    }
}
