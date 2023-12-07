use std::{cmp::Ordering, collections::BTreeMap};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            invalid => unreachable!("Invalid Input '{invalid}'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<&[Card]> for Hand {
    fn from(cards: &[Card]) -> Self {
        let cards_map = cards.iter().fold(BTreeMap::new(), |mut map, card| {
            map.entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            map
        });

        match cards_map.len() {
            1 => Hand::FiveOfKind,
            2 => match cards_map.values().max().unwrap() {
                4 => Hand::FourOfKind,
                3 => Hand::FullHouse,
                _ => unreachable!(),
            },
            3 => match cards_map.values().max().unwrap() {
                3 => Hand::ThreeOfKind,
                2 => Hand::TwoPairs,
                _ => unreachable!(),
            },
            4 => Hand::OnePair,
            5 => Hand::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Entry {
    cards: Vec<Card>,
    pid: usize,
    hand: Hand,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand.cmp(&other.hand) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(slf, other)| slf.cmp(other))
                .find(|&order| matches!(order, Ordering::Less | Ordering::Greater)),
        }
    }
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        let (cards, pid) = value.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(Card::from).collect();
        let pid = pid.parse().unwrap();
        let hand = Hand::from(cards.as_slice());

        Entry { cards, pid, hand }
    }
}

fn calc_total_winnings(input: &str) -> usize {
    let mut entries: Vec<_> = input.lines().map(Entry::from).collect();

    entries.sort_unstable();

    entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| entry.pid * (idx + 1))
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_total_winnings(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "07");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_solution() {
        assert_eq!(calc_total_winnings(INPUT), 6440);
    }
}

