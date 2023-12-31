fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    rounds: Vec<Round>,
}

// implement from &str for Round
impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let (mut red, mut green, mut blue) = (0, 0, 0);

        let draws = s.split(',').map(|draw| draw.trim());
        for draw in draws {
            let (count, colour) = draw.split_once(' ').expect("<count> <colour>");
            let count: u32 = count.parse().expect("count is a number");
            match colour {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => panic!("unknown colour"),
            }
        }

        Round { red, green, blue }
    }
}

// implement from &str for Game
impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let rounds_info = s.split_once(':').expect("Game <id>: <rounds_data>").1;

        let rounds = rounds_info.split(';').map(Round::from).collect();
        Game { rounds }
    }
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from)
        .map(|game| {
            let mut min_round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };

            game.rounds.iter().for_each(|round| {
                min_round.red = min_round.red.max(round.red);
                min_round.green = min_round.green.max(round.green);
                min_round.blue = min_round.blue.max(round.blue);
            });

            min_round.power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 2286);
    }
}
