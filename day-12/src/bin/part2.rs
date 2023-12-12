fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn arrangements(springs: &[char], groups: &[usize], total: u32) -> u32 {
    let spring_groups = springs
        .split(|c| *c != '#')
        .filter_map(|group| {
            if group.is_empty() {
                None
            } else {
                Some(group.len())
            }
        })
        .collect::<Vec<usize>>();

    if spring_groups == groups {
        return 1;
    }
    if !springs.contains(&'?') {
        // dbg!(spring_groups);
        return 0;
    }
    // if spring_groups.len() > groups.len() {
    //     // not sure if this is always correct
    //     // hopefully it is if we always add '#' first
    //     return 0;
    // }

    // max spring group is larger than max group:
    if spring_groups.iter().max() > groups.iter().max() {
        return 0;
    }

    // not enough '?' and '#' to fill the groups:
    if spring_groups.iter().sum::<usize>() + springs.iter().filter(|&&c| c == '?').count()
        < groups.iter().sum::<usize>()
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

    total + arrangements(&springs_hash, groups, total) + arrangements(&springs_dot, groups, total)
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, groups) = line.split_once(' ').expect("<springs> <groups>");
    let springs: Vec<char> = springs.chars().collect();
    let groups: Vec<usize> = groups
        .split(',')
        .map(|s| s.parse().expect("numbers"))
        .collect();
    (springs, groups)
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = parse_line(line);
            arrangements(&springs, &groups, 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines: Vec<(Vec<char>, Vec<usize>)> = include_str!("example.txt")
            .lines()
            .map(parse_line)
            .collect();

        assert_eq!(arrangements(&lines[0].0, &lines[0].1, 0), 1);
        assert_eq!(arrangements(&lines[1].0, &lines[1].1, 0), 4);
        assert_eq!(arrangements(&lines[2].0, &lines[2].1, 0), 1);
        assert_eq!(arrangements(&lines[3].0, &lines[3].1, 0), 1);
        assert_eq!(arrangements(&lines[4].0, &lines[4].1, 0), 4);
        assert_eq!(arrangements(&lines[5].0, &lines[5].1, 0), 10);

        assert_eq!(part2(include_str!("example.txt")), 21);
    }
}
