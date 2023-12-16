use indexmap::IndexMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[allow(non_snake_case)]
fn HASH(s: &str) -> usize {
    let mut current_value = 0;

    s.chars().for_each(|c| {
        dbg!(c);
        dbg!(c as usize);
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
        dbg!(current_value);
    });
    current_value
}

fn part2(input: &str) -> usize {
    // create 256 empty indexmaps in a vector:
    let mut boxes: Vec<IndexMap<&str, usize>> = vec![IndexMap::new(); 256];

    input
        .lines()
        .next()
        .expect("first line")
        .split(',')
        .for_each(|s| {
            let label = s
                .split(|c| c == '-' || c == '=')
                .next()
                .expect("label before '-' or '='");
            let hash = HASH(label);
            let current_box = &mut boxes[hash];
            match s.chars().last() {
                Some('-') => {
                    current_box.shift_remove(label);
                }
                lens_power @ Some('1'..='9') => {
                    current_box.insert(
                        label,
                        lens_power
                            .expect("lens_power char")
                            .to_digit(10)
                            .expect("lens_power is a digit") as usize,
                    );
                }
                _ => unreachable!(),
            }
        });

    boxes
        .iter()
        .enumerate()
        .map(|(box_i, box_)| {
            box_.iter()
                .enumerate()
                .map(|(slot_i, (_, lens_power))| (box_i + 1) * (slot_i + 1) * lens_power)
                .sum::<usize>()

            // println!("{}: {:?}", i, box_);
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 145);
    }
}
