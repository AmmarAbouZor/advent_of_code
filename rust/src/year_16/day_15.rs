use std::{num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone)]
struct Disk {
    count: usize,
    pos: usize,
}

impl FromStr for Disk {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = line.split_whitespace().collect();
        let count = parts[3].parse()?;
        let pos = parts.last().unwrap().trim_end_matches('.').parse()?;

        Ok(Disk { count, pos })
    }
}

impl Disk {
    fn new(count: usize, pos: usize) -> Self {
        Self { count, pos }
    }

    fn apply_move(&mut self) {
        self.pos += 1;
        if self.pos >= self.count {
            self.pos = 0;
        }
    }

    fn is_zero(&self) -> bool {
        self.pos == 0
    }
}

fn fetch_input() -> Vec<Disk> {
    read_text_from_file("16", "15")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn can_pass(disks: &[Disk]) -> bool {
    if !disks[0].is_zero() {
        return false;
    }
    let mut disks = disks.to_owned();
    for i in 1..disks.len() {
        disks.iter_mut().for_each(|disk| disk.apply_move());
        if !disks[i].is_zero() {
            return false;
        }
    }

    true
}

fn get_first_pass(mut disks: Vec<Disk>) -> usize {
    let mut count = 0;

    loop {
        disks.iter_mut().for_each(|disk| disk.apply_move());
        if can_pass(&disks) {
            return count;
        }
        count += 1;
    }
}

fn part_1() {
    let disks = fetch_input();

    let count = get_first_pass(disks);

    println!("part_1: solved in {count}");
}

fn part_2() {
    let mut disks = fetch_input();

    disks.push(Disk::new(11, 0));
    let count = get_first_pass(disks);

    println!("part_2: solved in {count}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_pass() {
        let input = vec![Disk::new(5, 4), Disk::new(2, 1)];

        assert_eq!(get_first_pass(input), 5);
    }
}
