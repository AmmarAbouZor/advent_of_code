use std::{cmp::Ordering, fmt::Display, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Eq)]
enum Entry {
    List(Vec<Entry>),
    Val(u8),
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::List(v) => write!(f, "{:?}", v),
            Entry::Val(v) => write!(f, "{}", v),
        }
    }
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse_from_chars(chars: &Vec<char>, mut index: usize) -> (Entry, usize) {
            let mut children = Vec::new();

            while chars[index] != ']' {
                match chars[index] {
                    ',' => index += 1,
                    '[' => {
                        index += 1;
                        let (entry, new_index) = parse_from_chars(chars, index);
                        index = new_index + 1;
                        children.push(entry);
                    }
                    ch if ch.is_ascii_digit() => {
                        let start_index = index;
                        while chars[index].is_numeric() {
                            index += 1;
                        }
                        let num = chars[start_index..index]
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap();
                        children.push(Entry::Val(num));
                    }
                    _ => unreachable!(),
                }
            }

            (Entry::List(children), index)
        }
        let chars: Vec<char> = input.chars().collect();

        let (entry, _) = parse_from_chars(&chars, 1);

        Ok(entry)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entry::Val(s), Entry::Val(o)) => s == o,
            (Entry::List(s), Entry::List(o)) => *s == *o,
            (Entry::List(s), Entry::Val(o)) => {
                let o = vec![Entry::Val(*o)];
                *s == o
            }
            (Entry::Val(s), Entry::List(o)) => {
                let s = vec![Entry::Val(*s)];
                s == *o
            }
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn cmp_lists(s: &[Entry], o: &[Entry]) -> Option<Ordering> {
            let mut ordering = Ordering::Equal;

            let mut s_iter = s.iter();
            let mut o_iter = o.iter();

            while ordering == Ordering::Equal {
                let (s_option, o_option) = (s_iter.next(), o_iter.next());
                if let (Some(s_item), Some(o_item)) = (s_option, o_option) {
                    ordering = s_item.partial_cmp(o_item).unwrap();
                } else if s_option.is_some() {
                    ordering = Ordering::Greater;
                } else if o_option.is_some() {
                    ordering = Ordering::Less;
                } else {
                    break;
                }
            }

            Some(ordering)
        }

        match (self, other) {
            (Entry::Val(s), Entry::Val(o)) => s.partial_cmp(o),
            (Entry::List(s), Entry::List(o)) => cmp_lists(s, o),
            (Entry::List(s), Entry::Val(o)) => {
                let o = vec![Entry::Val(*o)];
                cmp_lists(s, &o)
            }
            (Entry::Val(s), Entry::List(o)) => {
                let s = vec![Entry::Val(*s)];
                cmp_lists(&s, o)
            }
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Pair {
    left: Entry,
    right: Entry,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.left, self.right)
    }
}

impl From<&str> for Pair {
    fn from(lines: &str) -> Self {
        let (left, right) = lines.split_once('\n').unwrap();
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();

        Pair { left, right }
    }
}

fn calc_sum_valid_pair(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Pair::from)
        .enumerate()
        .filter(|(_, pair)| pair.left < pair.right)
        .map(|(index, _)| index + 1)
        .sum()
}

fn calc_dicoder_key(input: &str) -> usize {
    let mut packets: Vec<Entry> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let pack_2 = Entry::List(vec![Entry::List(vec![Entry::Val(2)])]);
    let pack_6 = Entry::List(vec![Entry::List(vec![Entry::Val(6)])]);

    packets.push(pack_2.clone());
    packets.push(pack_6.clone());

    packets.sort();

    let pack_2_index = packets.iter().position(|entry| *entry == pack_2).unwrap();
    let pack_6_index = packets.iter().position(|entry| *entry == pack_6).unwrap();

    (pack_2_index + 1) * (pack_6_index + 1)
}

fn part_1() {
    let input = read_text_from_file("22", "13");

    let answer = calc_sum_valid_pair(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "13");

    let answer = calc_dicoder_key(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_sum_valid_pair(INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_dicoder_key(INPUT), 140);
    }
}
