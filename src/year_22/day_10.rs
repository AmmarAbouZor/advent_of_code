use std::{num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Instr {
    Noop,
    Add(isize),
}

impl Instr {
    fn get_duration(&self) -> usize {
        match self {
            Instr::Noop => 1,
            Instr::Add(_) => 2,
        }
    }
}

impl FromStr for Instr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Instr::Noop),
            _ => {
                let val = s.split_whitespace().nth(1).unwrap().parse()?;
                Ok(Instr::Add(val))
            }
        }
    }
}

#[derive(Debug, Default)]
struct Proc {
    register: isize,
    counter: isize,
    strengths: Vec<isize>,
    pixels: Vec<char>,
}

const MEASURES: [isize; 6] = [20, 60, 100, 140, 180, 220];

impl Proc {
    fn new() -> Self {
        Proc {
            register: 1,
            ..Default::default()
        }
    }

    fn apply_strength(&mut self, ins: &Instr) {
        for _ in 0..ins.get_duration() {
            self.cycle_strength();
        }

        if let Instr::Add(val) = ins {
            self.register += val;
        }
    }

    fn cycle_strength(&mut self) {
        self.counter += 1;
        if MEASURES.contains(&self.counter) {
            self.strengths.push(self.counter * self.register);
        }
    }

    fn cycle_draw(&mut self) {
        self.counter += 1;

        let counter = self.counter % 40;
        let pixel = if (self.register..self.register + 3).contains(&counter) {
            '#'
        } else {
            ','
        };

        self.pixels.push(pixel);
    }

    fn apply_draw(&mut self, ins: &Instr) {
        for _ in 0..ins.get_duration() {
            self.cycle_draw();
        }

        if let Instr::Add(val) = ins {
            self.register += val;
        }
    }

    fn print(&self) {
        self.pixels
            .chunks(40)
            .for_each(|line| println!("{}", line.iter().collect::<String>()));
    }
}

fn calc_sum_signals(input: &str) -> isize {
    let mut proc = Proc::new();
    input
        .lines()
        .map(|line| line.parse::<Instr>().unwrap())
        .for_each(|ins| proc.apply_strength(&ins));

    assert!(proc.strengths.len() >= 6);
    proc.strengths.iter().sum()
}

fn part_1() {
    let input = read_text_from_file("22", "10");

    let answer = calc_sum_signals(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "10");

    let mut proc = Proc::new();
    input
        .lines()
        .map(|line| line.parse::<Instr>().unwrap())
        .for_each(|ins| proc.apply_draw(&ins));

    proc.print();
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let answer = calc_sum_signals(INPUT);
        assert_eq!(answer, 13140);
    }

    const INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
