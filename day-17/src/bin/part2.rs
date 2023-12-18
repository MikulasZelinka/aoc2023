use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    vec,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_possible(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
            Direction::Left | Direction::Right => vec![Direction::Down, Direction::Up],
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct State {
    row: isize,
    col: isize,
    heat_loss: u32,
    next_directions: Vec<Direction>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &str) -> u32 {
    input.trim().to_string();

    let city: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("1 .. 9"))
                .collect()
        })
        .collect();

    let mut queue = BinaryHeap::new();

    let initial_state = State {
        row: 0,
        col: 0,
        heat_loss: 0,
        next_directions: vec![Direction::Down, Direction::Right],
    };

    queue.push(initial_state.clone());

    let mut visited_states = HashSet::new();

    let mut states_in_queue = HashMap::new();

    while !queue.is_empty() {
        let state = queue.pop().expect("queue is not empty");

        visited_states.insert((state.row, state.col, state.next_directions.clone()));

        if state.row == city.len() as isize - 1 && state.col == city[0].len() as isize - 1 {
            return state.heat_loss;
        }

        for direction in state.next_directions {
            for move_length in 4..=10 {
                let (next_row, next_col) = match direction {
                    Direction::Up => (state.row - move_length, state.col),
                    Direction::Down => (state.row + move_length, state.col),
                    Direction::Left => (state.row, state.col - move_length),
                    Direction::Right => (state.row, state.col + move_length),
                };

                if next_row < 0
                    || next_row >= city.len() as isize
                    || next_col < 0
                    || next_col >= city[0].len() as isize
                {
                    continue;
                }

                let mut next_heat_loss = state.heat_loss;

                match direction {
                    Direction::Up => {
                        for i in 1..=move_length {
                            next_heat_loss += city[(state.row - i) as usize][state.col as usize]
                        }
                    }
                    Direction::Down => {
                        for i in 1..=move_length {
                            next_heat_loss += city[(state.row + i) as usize][state.col as usize]
                        }
                    }
                    Direction::Left => {
                        for i in 1..=move_length {
                            next_heat_loss += city[state.row as usize][(state.col - i) as usize]
                        }
                    }
                    Direction::Right => {
                        for i in 1..=move_length {
                            next_heat_loss += city[state.row as usize][(state.col + i) as usize]
                        }
                    }
                }

                let next_directions = direction.next_possible();

                let next_state = State {
                    row: next_row,
                    col: next_col,
                    heat_loss: next_heat_loss,
                    next_directions,
                };

                if visited_states.contains(&(
                    next_state.row,
                    next_state.col,
                    next_state.next_directions.clone(),
                )) {
                    continue;
                }

                if let Some(lowest_heat_loss_so_far) = states_in_queue.get(&(
                    next_state.row,
                    next_state.col,
                    next_state.next_directions.clone(),
                )) {
                    if next_state.heat_loss >= *lowest_heat_loss_so_far {
                        continue;
                    }
                }
                states_in_queue.insert(
                    (
                        next_state.row,
                        next_state.col,
                        next_state.next_directions.clone(),
                    ),
                    next_state.heat_loss,
                );

                queue.push(next_state);

                // dbg!(&next_state);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 94);
    }
}
