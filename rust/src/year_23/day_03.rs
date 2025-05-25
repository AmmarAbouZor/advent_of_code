use std::collections::{BTreeMap, BTreeSet};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    start_pos: Point,
    length: usize,
}

impl Number {
    fn new(value: usize, start_pos: Point, length: usize) -> Self {
        Self {
            value,
            start_pos,
            length,
        }
    }

    fn is_valid(&self, symb_set: &BTreeSet<Point>) -> bool {
        (self.start_pos.row.saturating_sub(1)..self.start_pos.row + 2)
            .flat_map(|row| {
                (self.start_pos.col.saturating_sub(1)..self.start_pos.col + self.length + 1)
                    .map(move |col| Point::new(row, col))
            })
            .any(|point| symb_set.contains(&point))
    }

    fn get_gear_pos(&self, gear_set: &BTreeSet<Point>) -> Option<Point> {
        (self.start_pos.row.saturating_sub(1)..self.start_pos.row + 2)
            .flat_map(|row| {
                (self.start_pos.col.saturating_sub(1)..self.start_pos.col + self.length + 1)
                    .map(move |col| Point::new(row, col))
            })
            .find(|point| gear_set.contains(point))
    }
}

#[derive(Debug)]
struct ParsedInput {
    numbers: Vec<Number>,
    symb_positions: BTreeSet<Point>,
    gear_positions: BTreeSet<Point>,
}

impl From<&str> for ParsedInput {
    fn from(input: &str) -> Self {
        let mut last_start_pos = None;
        let mut last_num_chars = Vec::new();

        let mut numbers: Vec<Number> = Vec::new();
        let mut symb_positions = BTreeSet::new();
        let mut gear_positions = BTreeSet::new();

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                let mut is_digit = false;
                match ch {
                    '.' => {}
                    ch if ch.is_ascii_digit() => {
                        is_digit = true;
                        if last_start_pos.is_none() {
                            last_start_pos = Some(Point::new(row, col));
                        }
                        last_num_chars.push(ch);
                    }
                    ' ' => unreachable!("invalid input space"),
                    ch if ch.is_alphabetic() => unreachable!("invalid input char: {ch}"),
                    '*' => {
                        _ = symb_positions.insert(Point::new(row, col));
                        _ = gear_positions.insert(Point::new(row, col));
                    }
                    _symb => {
                        _ = symb_positions.insert(Point::new(row, col));
                    }
                }

                if !is_digit && last_start_pos.is_some() {
                    let len = last_num_chars.len();
                    let number = last_num_chars.iter().collect::<String>();
                    let number = number.parse().unwrap();
                    let start_pos = last_start_pos.take().unwrap();
                    last_num_chars.clear();
                    numbers.push(Number::new(number, start_pos, len));
                }
            })
        });

        ParsedInput {
            numbers,
            symb_positions,
            gear_positions,
        }
    }
}

fn get_valid_sum(input: &str) -> usize {
    let parsed = ParsedInput::from(input);

    parsed
        .numbers
        .iter()
        .filter(|num| num.is_valid(&parsed.symb_positions))
        .map(|num| num.value)
        .sum()
}

fn get_gear_sum(input: &str) -> usize {
    let parsed = ParsedInput::from(input);

    let mut sum = 0;
    let mut adjescant_gears = BTreeMap::new();

    parsed
        .numbers
        .iter()
        .filter_map(|num| {
            num.get_gear_pos(&parsed.gear_positions)
                .map(|gear| (gear, num.value))
        })
        .for_each(|(gear, current_val)| {
            if let Some(adjescant_val) = adjescant_gears.get(&gear) {
                sum += current_val * *adjescant_val;
            } else {
                adjescant_gears.insert(gear, current_val);
            }
        });

    sum
}

fn part_1(input: &str) {
    let answer_1 = get_valid_sum(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {
    let answer_2 = get_gear_sum(input);

    println!("Part 2 answer is {answer_2}");
}

pub fn run() {
    let input = read_text_from_file("23", "03");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_solution() {
        assert_eq!(get_valid_sum(INPUT), 4361);
        assert_eq!(get_gear_sum(INPUT), 467835);
    }
}
