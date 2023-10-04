use itertools::Itertools;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Answers<'a> {
    lines: Vec<&'a [u8]>,
}

impl<'a> From<&'a str> for Answers<'a> {
    fn from(value: &'a str) -> Self {
        let lines = value.lines().map(|line| line.as_bytes()).collect();

        Answers { lines }
    }
}

impl<'a> Answers<'a> {
    fn unique_answers_count(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|line| line.iter())
            .unique()
            .count()
    }

    fn intersect_answers(&self) -> usize {
        let mut intersect = self.lines[0].to_vec();
        for line in self.lines.iter().skip(1) {
            intersect.retain(|byte| line.contains(byte));
        }

        intersect.len()
    }
}

fn calc_unique_sum(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Answers::from)
        .map(|answers| answers.unique_answers_count())
        .sum()
}

fn calc_intersect_sum(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Answers::from)
        .map(|answers| answers.intersect_answers())
        .sum()
}

fn part_1() {
    let input = read_text_from_file("20", "06");
    let answer = calc_unique_sum(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "06");
    let answer = calc_intersect_sum(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_solution() {
        assert_eq!(calc_unique_sum(INPUT), 11);
        assert_eq!(calc_intersect_sum(INPUT), 6);
    }
}

