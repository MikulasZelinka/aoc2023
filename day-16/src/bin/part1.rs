use std::{collections::HashSet, vec};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// direction is an enum of up, down, left, right
// up means (-1, 0), down means (1, 0), left means (0, -1), right means (0, 1)
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, tile: char) -> Self {
        match tile {
            '/' => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            '\\' => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    row: isize,
    col: isize,
    dir: Direction,
}

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut energised = vec![vec![false; grid[0].len()]; grid.len()];

    let mut seen_beams = HashSet::new();

    let mut beams = vec![Beam {
        row: 0,
        col: -1,
        dir: Direction::Right,
    }];
    while !beams.is_empty() {
        let mut new_beams = vec![];
        for beam in beams {
            if let Some(e) = energised
                .get_mut(beam.row as usize)
                .and_then(|row| row.get_mut(beam.col as usize))
            {
                *e = true;
            };
            if !seen_beams.insert(beam) {
                continue;
            }

            let mut new_beam = beam;

            match new_beam.dir {
                Direction::Right => {
                    new_beam.col += 1;
                }
                Direction::Left => {
                    new_beam.col -= 1;
                }
                Direction::Up => {
                    new_beam.row -= 1;
                }
                Direction::Down => {
                    new_beam.row += 1;
                }
            }

            if let Some(tile) = grid
                .get(new_beam.row as usize)
                .and_then(|row| row.get(new_beam.col as usize))
            {
                match tile {
                    '.' => {
                        new_beams.push(new_beam);
                    }
                    '\\' | '/' => {
                        new_beams.push(Beam {
                            row: new_beam.row,
                            col: new_beam.col,
                            dir: new_beam.dir.turn(*tile),
                        })
                        // change direction
                    }
                    '|' => match new_beam.dir {
                        Direction::Right | Direction::Left => {
                            new_beam.dir = Direction::Up;
                            new_beams.push(new_beam);
                            new_beams.push(Beam {
                                row: new_beam.row,
                                col: new_beam.col,
                                dir: Direction::Down,
                            })
                        }
                        Direction::Up | Direction::Down => {
                            new_beams.push(new_beam);
                        }
                    },
                    '-' => match new_beam.dir {
                        Direction::Up | Direction::Down => {
                            new_beam.dir = Direction::Left;
                            new_beams.push(new_beam);
                            new_beams.push(Beam {
                                row: new_beam.row,
                                col: new_beam.col,
                                dir: Direction::Right,
                            })
                        }
                        Direction::Right | Direction::Left => {
                            new_beams.push(new_beam);
                        }
                    },
                    _ => unreachable!("unknown tile"),
                }
            }
        }
        beams = new_beams;
        dbg!(&beams);
    }
    energised.iter().flatten().filter(|&&e| e).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 46);
    }
}
