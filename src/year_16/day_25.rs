use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone)]
enum Command {
    Copy(String, Argument),
    Increase(String),
    Decrease(String),
    JumpIfNotZero(Argument, Argument),
    Output(String),
}

#[derive(Debug, Clone)]
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
                let offset = Argument::new(parts.next().unwrap());
                Ok(Command::JumpIfNotZero(reg, offset))
            }
            "out" => {
                let reg = parts.next().unwrap().to_owned();
                Ok(Command::Output(reg))
            }
            _ => unreachable!(),
        }
    }
}

impl Command {
    fn apply(&self, reg_map: &mut HashMap<String, i32>, output: &mut Vec<i32>) -> i32 {
        match self {
            Command::Copy(reg, arg) => {
                let value = arg.get_val(reg_map);
                let register = reg_map.entry(reg.to_owned()).or_insert(0);
                *register = value;

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
            Command::JumpIfNotZero(arg, offset) => {
                let val = match arg {
                    Argument::Register(name) => reg_map.get(name).unwrap().to_owned(),
                    Argument::Value(value) => value.to_owned(),
                };

                let off_val = match offset {
                    Argument::Register(name) => *reg_map.get(name).unwrap(),
                    Argument::Value(value) => value.to_owned(),
                };
                if val != 0 {
                    off_val
                } else {
                    1
                }
            }
            Command::Output(reg) => {
                let val = reg_map.get(reg).unwrap().to_owned();
                output.push(val);

                1
            }
        }
    }
}

fn fetch_commands() -> Vec<Command> {
    read_text_from_file("16", "25")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_1() {
    let cmds = fetch_commands();
    let mut count = 0i32;

    let bound = 30;
    loop {
        count += 1;
        let mut map = HashMap::new();
        map.insert("a".to_string(), count);
        let mut index = 0i32;
        let mut output = Vec::new();

        let mut escape = 0;
        while index < cmds.len() as i32 && output.len() < bound && escape < 200000 {
            index += cmds[index as usize].apply(&mut map, &mut output);
            escape += 1;
        }

        if output.len() == bound {
            dbg!(&output);

            if output
                .iter()
                .enumerate()
                .all(|(i, val)| *val == (i % 2) as i32)
            {
                println!("value is {count}");
                break;
            }
        }
    }
}

pub fn run() {
    part_1();
}
