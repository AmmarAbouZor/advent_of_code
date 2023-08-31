use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Command {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosLetter(char),
    Revers(usize, usize),
    Move(usize, usize),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "swap" if parts[1] == "position" => {
                let pos_1 = parts[2].parse()?;
                let pos_2 = parts[5].parse()?;
                Ok(Command::SwapPos(pos_1, pos_2))
            }
            "swap" if parts[1] == "letter" => {
                let char_1 = parts[2].chars().next().unwrap();
                let char_2 = parts[5].chars().next().unwrap();
                Ok(Command::SwapLetter(char_1, char_2))
            }
            "rotate" if parts[1] == "left" => Ok(Command::RotateLeft(parts[2].parse()?)),
            "rotate" if parts[1] == "right" => Ok(Command::RotateRight(parts[2].parse()?)),
            "rotate" if parts[1] == "based" => Ok(Command::RotatePosLetter(
                parts.last().and_then(|word| word.chars().next()).unwrap(),
            )),
            "reverse" => {
                let pos_1 = parts[2].parse()?;
                let pos_2 = parts[4].parse()?;
                Ok(Command::Revers(pos_1, pos_2))
            }
            "move" => {
                let pos_1 = parts[2].parse()?;
                let pos_2 = parts[5].parse()?;
                Ok(Command::Move(pos_1, pos_2))
            }
            _ => unreachable!("invalid input"),
        }
    }
}

impl Command {
    fn apply(&self, word: &mut Vec<char>) {
        match self {
            Command::SwapPos(x, y) => {
                word.swap(*x, *y);
            }
            Command::SwapLetter(x, y) => {
                let pos_x = word.iter().position(|ch| ch == x).unwrap();
                let pos_y = word.iter().position(|ch| ch == y).unwrap();
                word.swap(pos_x, pos_y);
            }
            Command::RotateLeft(steps) => word.rotate_left(*steps),
            Command::RotateRight(steps) => word.rotate_right(*steps),
            Command::RotatePosLetter(letter) => {
                let mut pos = word.iter().position(|ch| ch == letter).unwrap();
                if pos >= 4 {
                    pos += 1;
                }
                pos += 1;

                if pos > word.len() {
                    pos -= word.len();
                }

                word.rotate_right(pos);
            }
            Command::Revers(x, y) => {
                let test = &mut word[*x..=*y];
                test.reverse();
            }
            Command::Move(x, y) => {
                let ch = word.remove(*x);
                word.insert(*y, ch);
            }
        }
    }
}

fn read_commands() -> Vec<Command> {
    read_text_from_file("16", "21")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn scramble(input: &str, commands: &Vec<Command>) -> String {
    let mut chars: Vec<char> = input.chars().collect();

    for cmd in commands {
        cmd.apply(&mut chars);
    }

    chars.into_iter().collect()
}

fn part_1() {
    let commands = read_commands();

    let result = scramble("abcdefgh", &commands);

    println!("part_1: result is {result}");
}

fn part_2() {
    let commands = read_commands();

    let input = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let answer = input
        .iter()
        .permutations(input.len())
        .find(|chars| {
            scramble(
                chars.iter().copied().collect::<String>().as_str(),
                &commands,
            ) == *"fbgdceah"
        })
        .unwrap();

    println!(
        "part_2: answer is {}",
        answer.into_iter().collect::<String>()
    );
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_commands() -> Vec<Command> {
        let input = r"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4        
rotate left 1 step       
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b        
rotate based on position of letter d";

        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    #[test]
    fn test_commands() {
        let commands = get_test_commands();
        assert_eq!(scramble("abcde", &commands), *"decab");
    }
}
