#![allow(warnings, unused)]

use std::collections::HashMap;

use crate::utls::read_lines_from_file;

#[derive(Debug)]
enum Command {
    Half(String),
    Triple(String),
    Increment(String),
    Jump(i16),
    JumpIfEven(String, i16),
    JumpIfOne(String, i16),
}

impl From<String> for Command {
    fn from(line: String) -> Self {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "hlf" => Command::Half(parts[1].into()),
            "tpl" => Command::Triple(parts[1].into()),
            "inc" => Command::Increment(parts[1].into()),
            "jmp" => Command::Jump(parts[1].parse().unwrap()),
            "jie" => Command::JumpIfEven(
                parts[1].trim_end_matches(',').into(),
                parts[2].parse().unwrap(),
            ),
            "jio" => Command::JumpIfOne(
                parts[1].trim_end_matches(',').into(),
                parts[2].parse().unwrap(),
            ),
            _ => panic!("invalid input"),
        }
    }
}

impl Command {
    fn apply(&self, map: &mut HashMap<String, usize>) -> i16 {
        match self {
            Command::Half(regis) => {
                *map.get_mut(regis).unwrap() /= 2;
                1
            }
            Command::Triple(regis) => {
                *map.get_mut(regis).unwrap() *= 3;
                1
            }
            Command::Increment(regis) => {
                *map.get_mut(regis).unwrap() += 1;
                1
            }
            Command::Jump(offset) => *offset,
            Command::JumpIfEven(regis, offset) => {
                if map[regis] % 2 == 0 {
                    *offset
                } else {
                    1
                }
            }
            Command::JumpIfOne(regis, offset) => {
                if map[regis] == 1 {
                    *offset
                } else {
                    1
                }
            }
        }
    }
}

fn get_commands() -> Vec<Command> {
    read_lines_from_file(r"src/year_15/day_23.txt")
        .into_iter()
        .map(|line| line.into())
        .collect()
}

fn run_commands(commands: Vec<Command>, regisers: &mut HashMap<String, usize>) {
    let mut index = 0;
    while index >= 0 && index < commands.len() as i16 {
        index += commands[index as usize].apply(regisers);
    }
}

fn part_1() {
    let commands: Vec<Command> = get_commands();
    let mut registers = HashMap::from([("a".to_owned(), 0usize), ("b".to_owned(), 0usize)]);

    run_commands(commands, &mut registers);

    println!("value of register b is {}", registers["b"]);
}

fn part_2() {
    let commands: Vec<Command> = get_commands();
    let mut registers = HashMap::from([("a".to_owned(), 1usize), ("b".to_owned(), 0usize)]);

    run_commands(commands, &mut registers);

    println!("part_2: value of register b is {}", registers["b"]);
}
pub fn run() {
    part_1();
    part_2();
}
