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

impl<'a> Password<'a> {
    fn is_valid(&self) -> bool {
        let ch_count = self.text.chars().filter(|&ch| ch == self.ch).count();

        self.rng.contains(&ch_count)
    }
}

fn calc_valid_password(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(Password::is_valid)
        .count()
}

fn part_1() {
    let input = read_text_from_file("20", "02");
    let answer = calc_valid_password(input.as_str());

    println!("Part 1 answer is {answer}");
}
fn part_2() {}

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
        assert_eq!(calc_valid_password(INPUT), 2)
    }
}

