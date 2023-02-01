use std::str::FromStr;

use crate::utls::read_text_from_file;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn get_value(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl From<char> for Choice {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => unreachable!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Round {
    player: Choice,
    enemy: Choice,
}

impl Round {
    fn play(&self) -> usize {
        let fight_result = match (self.player, self.enemy) {
            (p, e) if p == e => 3,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => 6,
            _ => 0,
        };

        fight_result + self.player.get_value()
    }

    fn parse_to_target(line: &str) -> Round {
        let mut chars = line.chars();

        let enemy: Choice = chars.next().unwrap().into();
        let target: Target = chars.nth(1).unwrap().into();

        let player = match target {
            Target::Draw => enemy,
            Target::Lose => match enemy {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Target::Win => match enemy {
                Choice::Scissors => Choice::Rock,
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
            },
        };

        Round { player, enemy }
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let enemy = chars.next().unwrap().into();
        let player = chars.nth(1).unwrap().into();

        Ok(Round { player, enemy })
    }
}

enum Target {
    Lose,
    Draw,
    Win,
}

impl From<char> for Target {
    fn from(value: char) -> Self {
        match value {
            'X' => Target::Lose,
            'Y' => Target::Draw,
            'Z' => Target::Win,
            _ => unreachable!(),
        }
    }
}

fn part_1() {
    let score: usize = read_text_from_file("22", "02")
        .lines()
        .map(|line| line.parse::<Round>().unwrap())
        .map(|round| round.play())
        .sum();

    println!("Score of part one is {score}");
}

fn part_2() {
    let score: usize = read_text_from_file("22", "02")
        .lines()
        .map(Round::parse_to_target)
        .map(|round| round.play())
        .sum();

    println!("Score of part two is {score}");
}

pub fn run() {
    part_1();
    part_2();
}
