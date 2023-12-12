use cached::proc_macro::cached;
use cached::UnboundCache;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use rayon::{iter::ParallelIterator, str::ParallelString};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

type CachedFn = (Vec<char>, Vec<usize>, usize);

fn num_arrangements_to_cacheable(
    springs: &[char],
    groups: &[usize],
    num_completed_groups: usize,
    current_group_len: usize,
) -> CachedFn {
    (
        springs.to_vec(),
        groups[num_completed_groups..].to_vec(),
        current_group_len,
    )
}

#[cached(
    type = "UnboundCache<CachedFn, u32>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ num_arrangements_to_cacheable(springs, groups, num_completed_groups, current_group_len) }"#
)]
fn num_arrangements(
    springs: &[char],
    groups: &[usize],
    num_completed_groups: usize,
    current_group_len: usize,
    _bar: &ProgressBar,
) -> u32 {
    if springs.is_empty() {
        // special case, handle if the whole spring sequence just ended and we just got a valid arrangement
        // altneratively, we could just add a '.' to the end of the spring sequence
        if num_completed_groups == groups.len() - 1
            && current_group_len == groups[num_completed_groups]
        {
            _bar.inc(1);
            return 1;
        }
        let result = num_completed_groups == groups.len();
        if result {
            _bar.inc(1);
        }
        return result as u32;
    }

    let (current_spring, next_springs) = (springs[0], &springs[1..]);

    match current_spring {
        // '.' means that a group has ended (if it existed)
        '.' => {
            match current_group_len {
                // no group to end, continue
                0 => num_arrangements(
                    next_springs,
                    groups,
                    num_completed_groups,
                    current_group_len,
                    _bar,
                ),
                // successfully completed a group
                n if n == groups[num_completed_groups] => {
                    num_arrangements(&springs[1..], groups, num_completed_groups + 1, 0, _bar)
                }
                // group ended prematurely
                _ => 0,
            }
        }
        // '#' means that a group has started or is continuing
        '#' => {
            if num_completed_groups == groups.len() {
                // too many groups
                0
            } else {
                num_arrangements(
                    &springs[1..],
                    groups,
                    num_completed_groups,
                    current_group_len + 1,
                    _bar,
                )
            }
        }

        '?' => {
            // recursively try both possibilities by prepending '.' and '#'
            num_arrangements(
                &[&['.'], next_springs].concat(),
                groups,
                num_completed_groups,
                current_group_len,
                _bar,
            ) + num_arrangements(
                &[&['#'], next_springs].concat(),
                groups,
                num_completed_groups,
                current_group_len,
                _bar,
            )
        }

        _ => unreachable!(),
    }
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
            let result = num_arrangements(&springs, &groups, 0, 0, &bar);
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
    }
}
