use std::ops::RangeInclusive;

use crate::utls::read_text_from_file;

struct Password<'a> {
    rng: RangeInclusive<usize>,
    ch: char,
    text: &'a str,
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(line: &'a str) -> Self {
        let mut parts = line.split_whitespace();
        let rng = parts.next().unwrap();
        let (min, max) = rng.split_once('-').unwrap();
        let rng = RangeInclusive::new(min.parse().unwrap(), max.parse().unwrap());
        let ch = parts.next().and_then(|chs| chs.chars().next()).unwrap();
        let text = parts.next().unwrap();

        Password { rng, ch, text }
    }
}

impl Password<'_> {
    fn is_valid_1(&self) -> bool {
        let ch_count = self.text.chars().filter(|&ch| ch == self.ch).count();

        self.rng.contains(&ch_count)
    }

    fn is_valid_2(&self) -> bool {
        let mut chars = self.text.chars();
        let start = *self.rng.start();
        let end = *self.rng.end();
        if let (Some(ch_1), Some(ch_2)) = (chars.nth(start - 1), chars.nth(end - 1 - start)) {
            ch_1 != ch_2 && (self.ch == ch_1 || self.ch == ch_2)
        } else {
            false
        }
    }
}

fn calc_valid_password_1(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(Password::is_valid_1)
        .count()
}

fn calc_valid_password_2(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(Password::is_valid_2)
        .count()
}

fn part_1() {
    let input = read_text_from_file("20", "02");
    let answer = calc_valid_password_1(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "02");
    let answer = calc_valid_password_2(input.as_str());

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_valid_password_1(INPUT), 2);
        assert_eq!(calc_valid_password_2(INPUT), 1);
    }
}
