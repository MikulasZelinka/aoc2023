use itertools::Itertools;
use rangemap::RangeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = lines
        .next()
        .expect("first line")
        .split_once(':')
        .expect("seeds: <seeds>")
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("seeds are numbers"))
        .collect();

    // this naive expansion approach took 1m56s to run
    let mut expanded_seeds = vec![];
    // take always two seeds, and expand them
    for (seed, range) in seeds.iter().tuples() {
        // add seed, seed + 1, ... seed + range - 1 to expanded_seeds
        expanded_seeds.extend(*seed..seed + range);
    }
    seeds = expanded_seeds;

    // 1_917_300_386 seeds
    dbg!(seeds.len());

    let lines = lines.collect::<Vec<_>>();
    let map_segments: Vec<&[&str]> = lines
        .split(|line| line.is_empty())
        .filter(|block| !block.is_empty())
        .collect();
    // dbg!(map_segments);

    // map_segment is:
    // destination_start, source_start, range_length
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    for map_segment in map_segments {
        let mut map = RangeMap::new();
        let mut map_segment = map_segment.iter();
        map_segment.next();
        map_segment.for_each(|line| {
            let (destination_start, source_start, range_length) = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().expect("numbers"))
                .collect_tuple()
                .expect("destination_start, source_start, range_length");

            map.insert(source_start..source_start + range_length, destination_start);
        });
        let mut new_seeds = vec![];

        for value in seeds.iter() {
            match map.get_key_value(value) {
                Some((range, destination_start)) => {
                    new_seeds.push(*destination_start + value - range.start);
                }
                None => {
                    new_seeds.push(*value);
                }
            }
        }
        seeds = new_seeds;
    }

    *seeds.iter().min().expect("min")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 46);
    }
}
