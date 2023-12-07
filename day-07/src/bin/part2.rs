use counter::Counter;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
    // 248722222
    // too low
}

const CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
    bid: u32,
}

// implement from &str for Hand
impl Hand {
    fn from(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').expect("<cards> <bid>");
        let bid = bid.parse().expect("bid number");
        Hand {
            cards: cards
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("5 cards"),
            bid,
        }
    }
}

impl Hand {
    fn most_common(&self) -> Vec<u32> {
        let mut top_two: Vec<u32> = Counter::init(self.cards)
            .k_most_common_ordered(2)
            .iter()
            .map(|(_char, count)| *count)
            .collect();

        // add the number of Js to the top count
        top_two[0] += self.cards.iter().filter(|&c| *c == 'J').count() as u32;
        top_two[0] = top_two[0].min(5);
        top_two
    }

    fn card_ranks(&self) -> Vec<usize> {
        self.cards
            .iter()
            .map(|c| CARDS.iter().position(|&x| x == *c).unwrap())
            .collect::<Vec<usize>>()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.most_common().cmp(&other.most_common()) {
            std::cmp::Ordering::Equal => self.card_ranks().cmp(&other.card_ranks()),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::from).collect::<Vec<Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| {
            // dbg!(rank, hand);
            hand.bid * (rank as u32 + 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::from("32T3K 765");
        let hand2 = Hand::from("KK677 28");
        let hand3 = Hand::from("T55J5 684");
        let hand4 = Hand::from("QQQJA 483");
        let hand5 = Hand::from("KTJJT 220");

        dbg!(hand1.most_common());
        dbg!(hand2.most_common());
        dbg!(hand3.most_common());
        dbg!(hand4.most_common());
        dbg!(hand5.most_common());

        assert!(hand1 < hand2);
        assert!(hand2 < hand3);
        assert!(hand3 < hand4);
        assert!(hand4 < hand5);
    }

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 5905);
    }
}
