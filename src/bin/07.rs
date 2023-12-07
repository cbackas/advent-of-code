use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    card_strengths: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_strengths == other.card_strengths
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type.clone() as u8;
        let other_type = other.hand_type.clone() as u8;
        self_type
            .cmp(&other_type)
            .then_with(|| self.card_strengths.cmp(&other.card_strengths))
    }
}

fn map_card_strengths(cards: &str, wildcard_joker: bool) -> Vec<u32> {
    cards
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => match wildcard_joker {
                true => 0,
                false => 11,
            },
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        })
        .collect::<Vec<u32>>()
}

fn identify_hand_type(counts: HashMap<&u32, usize>) -> HandType {
    match counts.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => {
            let has_triplet = counts.values().any(|&v| v == 3);
            let has_pair = counts.values().any(|&v| v == 2);
            if has_triplet && has_pair {
                HandType::FullHouse
            } else if has_triplet {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPairs
            }
        }
        2 => {
            let has_quadruplet = counts.values().any(|&v| v == 4);
            if has_quadruplet {
                HandType::FourOfAKind
            } else if counts.values().any(|&v| v == 3) {
                HandType::FullHouse
            } else {
                unreachable!("Invalid hand")
            }
        }
        _ => HandType::FiveOfAKind,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands: u32 = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut split| {
            let cards = split.next().unwrap();
            let bid = split.next().unwrap();

            let card_strengths = map_card_strengths(cards, false);
            let counts = card_strengths.iter().counts();

            let hand_type = identify_hand_type(counts);

            Hand {
                card_strengths,
                hand_type,
                bid: bid.parse().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, hand)| {
            let multiplier = i + 1;
            let result = hand.bid * multiplier as u32;
            result
        })
        .sum();

    Some(hands as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: u32 = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut split| {
            let cards = split.next().unwrap();
            let bid = split.next().unwrap();

            let card_strengths = map_card_strengths(cards, true);
            let mut counts = card_strengths.iter().counts();

            // add the wildcards to the most common card
            {
                let wildcards = counts
                    .iter()
                    .find(|(&&k, _)| k == 0)
                    .map(|(_, &v)| v)
                    .unwrap_or(0)
                    .to_owned();
                if wildcards > 0 && wildcards < 5 {
                    let most_cards = counts
                        .iter()
                        .filter(|(&&key, _)| key != 0)
                        .max_by_key(|&(_, value)| value)
                        .unwrap();

                    let new_count = most_cards.1 + wildcards;

                    counts.insert(most_cards.0, new_count);
                    counts.remove(&0);
                }
            }

            let hand_type = identify_hand_type(counts);

            Hand {
                card_strengths,
                hand_type,
                bid: bid.parse().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, hand)| {
            let multiplier = i + 1;
            let result = hand.bid * multiplier as u32;
            result
        })
        .sum();

    Some(hands as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
