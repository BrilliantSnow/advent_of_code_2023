use std::{cmp::Ordering, collections::HashMap};

use crate::{benchmark_util::Solution, input_util};

pub struct Day07;

#[derive(PartialEq, Eq)]
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
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_compare = |left: &str, right: &str| {
            if let Some(first_diff) = left
                .chars()
                .zip(right.chars())
                .find(|(left, right)| left != right)
            {
                let first = PART2CARDS
                    .iter()
                    .enumerate()
                    .find(|(_, x)| first_diff.0.eq(x))
                    .unwrap()
                    .0;
                let second = PART2CARDS
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
        if self.rank() > other.rank() {
            Ordering::Greater
        } else if self.rank() < other.rank() {
            Ordering::Less
        } else {
            (hand_compare)(self.inner(), other.inner())
        }
    }
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const PART2CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

impl From<String> for PokerHand {
    fn from(value: String) -> Self {
        let mut letter_count: Vec<(char, i64)> = value
            .chars()
            .into_iter()
            .fold(HashMap::new(), |mut map, letter| {
                if let Some(exists) = map.get_mut(&letter) {
                    *exists += 1;
                } else {
                    map.insert(letter, 1);
                }
                map
            })
            .into_iter()
            .collect();
        println!("Letter occurances: {:?}", letter_count);
        letter_count.sort_by_key(|pair| pair.1);
        letter_count.reverse();

        let total_jokers = letter_count.iter().find(|x| x.0 == 'J').map(|x| x.1);
        let most_common_non_joker = letter_count.iter_mut().find(|x| x.0 != 'J');
        if let Some(total) = total_jokers {
            if let Some(add_to) = most_common_non_joker {
                add_to.1 += total;
                letter_count.remove(
                    letter_count
                        .iter()
                        .enumerate()
                        .find(|x| x.1 .0 == 'J')
                        .unwrap()
                        .0,
                );
            }
        }

        println!("Sorted letters: {:?}", letter_count);
        if letter_count.len() == 1 {
            return PokerHand::FiveOfAKind(value);
        }
        let most_popular = (letter_count.get(0).unwrap(), letter_count.get(1).unwrap());
        match most_popular {
            ((_, 5), _) => PokerHand::FiveOfAKind(value),
            ((_, 4), _) => PokerHand::FourOfAKind(value),
            ((_, 3), (_, 2)) => PokerHand::FullHouse(value),
            ((_, 3), _) => PokerHand::ThreeOfAKind(value),
            ((_, 2), (_, 2)) => PokerHand::TwoPair(value),
            ((_, 2), _) => PokerHand::OnePair(value),
            ((_, 1), _) => PokerHand::HighCard(value),
            _ => unreachable!("Hand has more than 5 cards??"),
        }
    }
}

impl Solution for Day07 {
    fn part_one(&self, file_path: &str) -> i64 {
        let mut poker_hands: Vec<(PokerHand, i64)> = input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let (hand, bet) = line.split_once(" ").unwrap();
                let bet: i64 = bet.parse().unwrap();
                (hand.to_owned(), bet)
            })
            .map(|(hand, bet)| (PokerHand::from(hand), bet))
            .collect();
        poker_hands.sort_by(|(hand, _), (hand2, _)| hand.cmp(hand2));
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
                let (hand, bet) = line.split_once(" ").unwrap();
                let bet: i64 = bet.parse().unwrap();
                (hand.to_owned(), bet)
            })
            .map(|(hand, bet)| (PokerHand::from(hand), bet))
            .collect();
        poker_hands.sort_by(|(hand, _), (hand2, _)| hand.cmp(hand2));
        poker_hands
            .iter()
            .enumerate()
            .map(|(ranking, (_, bet))| (ranking + 1) as i64 * bet)
            .sum()
    }
}
