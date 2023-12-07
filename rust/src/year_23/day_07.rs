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

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    cards: Vec<Card>,
    pid: usize,
    hand: Hand,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(slf, other)| slf.cmp(other))
                .find(|&order| matches!(order, Ordering::Less | Ordering::Greater))
                .unwrap(),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardJ {
    Jocker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for CardJ {
    fn from(value: char) -> Self {
        match value {
            '2' => CardJ::Two,
            '3' => CardJ::Three,
            '4' => CardJ::Four,
            '5' => CardJ::Five,
            '6' => CardJ::Six,
            '7' => CardJ::Seven,
            '8' => CardJ::Eight,
            '9' => CardJ::Nine,
            'T' => CardJ::Ten,
            'J' => CardJ::Jocker,
            'Q' => CardJ::Queen,
            'K' => CardJ::King,
            'A' => CardJ::Ace,
            invalid => unreachable!("Invalid Input '{invalid}'"),
        }
    }
}

impl CardJ {
    fn get_all_possibilities(&self) -> Vec<Card> {
        use Card as C;
        match self {
            CardJ::Jocker => vec![
                C::Two,
                C::Three,
                C::Four,
                C::Five,
                C::Six,
                C::Seven,
                C::Eight,
                C::Nine,
                C::Ten,
                C::Jack,
                C::Queen,
                C::King,
                C::Ace,
            ],
            CardJ::Two => vec![C::Two],
            CardJ::Three => vec![C::Three],
            CardJ::Four => vec![C::Four],
            CardJ::Five => vec![C::Five],
            CardJ::Six => vec![C::Six],
            CardJ::Seven => vec![C::Seven],
            CardJ::Eight => vec![C::Eight],
            CardJ::Nine => vec![C::Nine],
            CardJ::Ten => vec![C::Ten],
            CardJ::Queen => vec![C::Queen],
            CardJ::King => vec![C::King],
            CardJ::Ace => vec![C::Ace],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EntryJ {
    cards: Vec<CardJ>,
    pid: usize,
    best_hand: Hand,
}

impl Ord for EntryJ {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.best_hand.cmp(&other.best_hand) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(slf, other)| slf.cmp(other))
                .find(|&order| matches!(order, Ordering::Less | Ordering::Greater))
                .unwrap(),
        }
    }
}

impl PartialOrd for EntryJ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for EntryJ {
    fn from(value: &str) -> Self {
        let (cards, pid) = value.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(CardJ::from).collect();
        let pid = pid.parse().unwrap();
        let best_hand = Self::get_best_hand(&cards);

        EntryJ {
            cards,
            pid,
            best_hand,
        }
    }
}

impl EntryJ {
    fn get_best_hand(cards: &[CardJ]) -> Hand {
        let mut possiblites = vec![Vec::new()];
        for card_j in cards {
            let mut new_poss = Vec::new();
            while let Some(poss) = possiblites.pop() {
                for card in card_j.get_all_possibilities() {
                    let mut clone = poss.clone();
                    clone.push(card);
                    new_poss.push(clone);
                }
            }
            possiblites = new_poss;
        }

        possiblites
            .iter()
            .map(|cards| Hand::from(cards.as_slice()))
            .max()
            .unwrap()
    }
}

fn calc_jocker_winnings(input: &str) -> usize {
    let mut entries: Vec<_> = input.lines().map(EntryJ::from).collect();

    entries.sort_unstable();

    entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| entry.pid * (idx + 1))
        .sum()
}

fn part_2(input: &str) {
    let answer = calc_jocker_winnings(input);

    println!("Part 2 answer is {answer}");
}

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
        assert_eq!(calc_jocker_winnings(INPUT), 5905);
    }
}

