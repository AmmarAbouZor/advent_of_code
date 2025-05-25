use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Command {
    Copy(String, Argument),
    Increase(String),
    Decrease(String),
    JumpIfZero(Argument, i32),
}

#[derive(Debug)]
enum Argument {
    Register(String),
    Value(i32),
}

impl Argument {
    fn new(input: &str) -> Self {
        if let Ok(num) = input.parse::<i32>() {
            Argument::Value(num)
        } else {
            Argument::Register(input.into())
        }
    }
}

impl Argument {
    fn get_val(&self, reg_map: &HashMap<String, i32>) -> i32 {
        match self {
            Argument::Register(name) => reg_map.get(name).unwrap().to_owned(),
            Argument::Value(val) => val.to_owned(),
        }
    }
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        match parts.next().unwrap() {
            "cpy" => {
                let arg = Argument::new(parts.next().unwrap());
                let register = parts.next().unwrap().to_owned();

                Ok(Command::Copy(register, arg))
            }
            "inc" => {
                let reg = parts.next().unwrap().to_owned();
                Ok(Command::Increase(reg))
            }
            "dec" => {
                let reg = parts.next().unwrap().to_owned();
                Ok(Command::Decrease(reg))
            }
            "jnz" => {
                let reg = Argument::new(parts.next().unwrap());
                let offset = parts.next().unwrap().parse()?;
                Ok(Command::JumpIfZero(reg, offset))
            }
            _ => unreachable!(),
        }
    }
}

impl Command {
    fn apply(&self, reg_map: &mut HashMap<String, i32>) -> i32 {
        match self {
            Command::Copy(reg, arg) => {
                let value = arg.get_val(reg_map);
                let register = reg_map.entry(reg.to_owned()).or_insert(0);
                *register = value.to_owned();

                1
            }
            Command::Increase(reg) => {
                let register = reg_map.entry(reg.to_owned()).or_insert(0);
                *register += 1;

                1
            }
            Command::Decrease(reg) => {
                let register = reg_map.entry(reg.to_owned()).or_insert(0);
                *register -= 1;

                1
            }
            Command::JumpIfZero(arg, offset) => {
                let val = match arg {
                    Argument::Register(name) => *reg_map.get(name).unwrap_or(&0),
                    Argument::Value(value) => value.to_owned(),
                };

                if val != 0 { *offset } else { 1 }
            }
        }
    }
}

fn parse_input() -> Vec<Command> {
    read_text_from_file("16", "12")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_1() {
    let cmds = parse_input();
    let mut index = 0i32;
    let mut reg_map = HashMap::new();

    while index < cmds.len() as i32 {
        index += cmds[index as usize].apply(&mut reg_map);
    }

    println!("part 1: value in reg a is {}", reg_map.get("a").unwrap());
}

fn part_2() {
    let cmds = parse_input();
    let mut index = 0i32;
    let mut reg_map = HashMap::new();
    reg_map.insert("c".to_owned(), 1);

    while index < cmds.len() as i32 {
        index += cmds[index as usize].apply(&mut reg_map);
    }

    println!("part 2: value in reg a is {}", reg_map.get("a").unwrap());
}

pub fn run() {
    part_1();
    part_2();
}
