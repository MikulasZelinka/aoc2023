use itertools::Itertools;

#[allow(dead_code)]
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, 1);
    dbg!(output);
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

pub fn part1(input: &str, expansion: usize) -> usize {
    let mut galaxies: Vec<Pos> = vec![];
    let mut x_max = 0;
    let mut y_max = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Pos { x, y });
            }
            x_max = x_max.max(x);
        }
        y_max = y_max.max(y);
    }
    dbg!(&galaxies);

    let mut empty_cols = vec![
        true;
        galaxies
            .iter()
            .map(|p| p.x)
            .max()
            .expect("maximum x for galaxies")
            + 1
    ];
    let mut empty_rows = vec![
        true;
        galaxies
            .iter()
            .map(|p| p.y)
            .max()
            .expect("maximum y for galaxies")
            + 1
    ];

    for galaxy in &galaxies {
        empty_cols[galaxy.x] = false;
        empty_rows[galaxy.y] = false;
    }

    dbg!(&empty_cols);
    dbg!(&empty_rows);

    let mut sum_of_lengths = 0;

    for (g1, g2) in galaxies.iter().tuple_combinations() {
        let mut distance =
            ((g1.x as i32 - g2.x as i32).abs() + (g1.y as i32 - g2.y as i32).abs()) as usize;

        // @TODO: is there a better way of writing the slices?
        distance += empty_cols[g1.x.min(g2.x)..g1.x.max(g2.x)]
            .iter()
            .filter(|b| **b)
            .count()
            * expansion;
        distance += empty_rows[g1.y.min(g1.y)..g1.y.max(g2.y)]
            .iter()
            .filter(|b| **b)
            .count()
            * expansion;
        sum_of_lengths += distance;
    }

    sum_of_lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt"), 1), 374);
    }
}
