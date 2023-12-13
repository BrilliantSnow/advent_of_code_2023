use std::{cmp::Ordering, collections::HashMap};

use crate::{benchmark_util::Solution, input_util};

pub struct Day07;

#[derive(PartialEq, Eq, PartialOrd)]
enum PokerHand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl PokerHand {
    fn rank(&self) -> usize {
        match self {
            PokerHand::FiveOfAKind(_) => 6,
            PokerHand::FourOfAKind(_) => 5,
            PokerHand::FullHouse(_) => 4,
            PokerHand::ThreeOfAKind(_) => 3,
            PokerHand::TwoPair(_) => 2,
            PokerHand::OnePair(_) => 1,
            PokerHand::HighCard(_) => 0,
        }
    }
    fn inner(&self) -> &str {
        match self {
            PokerHand::FiveOfAKind(inner) => inner,
            PokerHand::FourOfAKind(inner) => inner,
            PokerHand::FullHouse(inner) => inner,
            PokerHand::ThreeOfAKind(inner) => inner,
            PokerHand::TwoPair(inner) => inner,
            PokerHand::OnePair(inner) => inner,
            PokerHand::HighCard(inner) => inner,
        }
    }

    fn build_sort(rankings: [char; 13], left: &PokerHand, right: &PokerHand) -> Ordering {
        let hand_compare = |left: &str, right: &str| {
            if let Some(first_diff) = left
                .chars()
                .zip(right.chars())
                .find(|(left, right)| left != right)
            {
                let first = rankings
                    .iter()
                    .enumerate()
                    .find(|(_, x)| first_diff.0.eq(x))
                    .unwrap()
                    .0;
                let second = rankings
                    .iter()
                    .enumerate()
                    .find(|(_, x)| first_diff.1.eq(x))
                    .unwrap()
                    .0;
                second.cmp(&first)
            } else {
                Ordering::Equal
            }
        };
        match left.rank().cmp(&right.rank()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => (hand_compare)(left.inner(), right.inner()),
        }
    }

    fn parse_part_one(value: String) -> Self {
        let letter_count: HashMap<char, i64> =
            value.chars().fold(HashMap::new(), |mut map, letter| {
                if let Some(exists) = map.get_mut(&letter) {
                    *exists += 1;
                } else {
                    map.insert(letter, 1);
                }
                map
            });

        // let jokers = letter_count.get(&'J').unwrap_or(&0);
        let max = letter_count
            .values()
            .max()
            .expect("Letter count never has a length of zero.");
        let size = letter_count.len();

        match (max, size) {
            (5, 1) => PokerHand::FiveOfAKind(value),
            (4, 2) => PokerHand::FourOfAKind(value),
            (3, 2) => PokerHand::FullHouse(value),
            (3, 3) => PokerHand::ThreeOfAKind(value),
            (2, 3) => PokerHand::TwoPair(value),
            (2, 4) => PokerHand::OnePair(value),
            (1, 5) => PokerHand::HighCard(value),
            _ => unreachable!("All combinations of 5 cards should be covered"),
        }
    }

    fn parse_part_two(value: String) -> Self {
        let letter_count: HashMap<char, i64> =
            value.chars().fold(HashMap::new(), |mut map, letter| {
                if let Some(exists) = map.get_mut(&letter) {
                    *exists += 1;
                } else {
                    map.insert(letter, 1);
                }
                map
            });

        let jokers = letter_count.get(&'J').unwrap_or(&0);
        let max = letter_count
            .values()
            .max()
            .expect("Letter count never has a length of zero.");
        let size = letter_count.len();

        match (max, size) {
            (5, _) => PokerHand::FiveOfAKind(value),
            (4, 2) => {
                if 0.eq(jokers) {
                    PokerHand::FourOfAKind(value)
                } else {
                    PokerHand::FiveOfAKind(value)
                }
            }
            (3, 2) => {
                if 0.eq(jokers) {
                    PokerHand::FullHouse(value)
                } else {
                    PokerHand::FiveOfAKind(value)
                }
            }
            (3, 3) => {
                if 0.eq(jokers) {
                    PokerHand::ThreeOfAKind(value)
                } else {
                    PokerHand::FourOfAKind(value)
                }
            }
            (2, 3) => {
                if 0.eq(jokers) {
                    PokerHand::TwoPair(value)
                } else if 1.eq(jokers) {
                    PokerHand::FullHouse(value)
                } else {
                    PokerHand::FourOfAKind(value)
                }
            }
            (2, 4) => {
                if 0.eq(jokers) {
                    PokerHand::OnePair(value)
                } else {
                    PokerHand::ThreeOfAKind(value)
                }
            }
            (1, 5) => {
                if 0.eq(jokers) {
                    PokerHand::HighCard(value)
                } else {
                    PokerHand::OnePair(value)
                }
            }
            _ => unreachable!("Did not cover case: {}", value),
        }
    }
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const PART2CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

impl Solution for Day07 {
    fn part_one(&self, file_path: &str) -> i64 {
        let mut poker_hands: Vec<(PokerHand, i64)> = input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let (hand, bet) = line.split_once(' ').unwrap();
                let bet: i64 = bet.parse().unwrap();
                (hand.to_owned(), bet)
            })
            .map(|(hand, bet)| (PokerHand::parse_part_one(hand), bet))
            .collect();
        poker_hands.sort_by(|(left, _), (right, _)| PokerHand::build_sort(CARDS, left, right));
        poker_hands
            .iter()
            .enumerate()
            .map(|(ranking, (_, bet))| (ranking + 1) as i64 * bet)
            .sum()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        let mut poker_hands: Vec<(PokerHand, i64)> = input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let (hand, bet) = line.split_once(' ').unwrap();
                let bet: i64 = bet.parse().unwrap();
                (hand.to_owned(), bet)
            })
            .map(|(hand, bet)| (PokerHand::parse_part_two(hand), bet))
            .collect();
        poker_hands.sort_by(|(left, _), (right, _)| PokerHand::build_sort(PART2CARDS, left, right));
        poker_hands
            .iter()
            .enumerate()
            .map(|(ranking, (_, bet))| (ranking + 1) as i64 * bet)
            .sum()
    }
}
