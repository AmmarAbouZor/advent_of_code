use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone)]
enum Command {
    Copy(String, Argument),
    Increase(String),
    Decrease(String),
    JumpIfNotZero(Argument, Argument),
    Toggle(String),
    Invalid(Argument, Argument),
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
            "tgl" => {
                let reg = parts.next().unwrap().to_owned();
                Ok(Command::Toggle(reg))
            }
            _ => unreachable!(),
        }
    }
}

impl Command {
    fn apply(
        &self,
        reg_map: &mut HashMap<String, i32>,
        cur_index: i32,
        commands: &mut [Command],
    ) -> i32 {
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
            Command::Toggle(reg) => {
                let register = reg_map.get(reg).unwrap();
                let target_index = cur_index + register;

                if let Some(cmd) = commands.get_mut(target_index as usize) {
                    let replace_cmd = match cmd {
                        Command::Increase(val) => Command::Decrease(val.clone()),
                        Command::Decrease(val) => Command::Increase(val.clone()),
                        Command::Copy(x, y) => {
                            Command::JumpIfNotZero(y.clone(), Argument::Register(x.clone()))
                        }
                        Command::JumpIfNotZero(x, y) => {
                            if let Argument::Register(reg) = y {
                                Command::Copy(reg.clone(), x.clone())
                            } else {
                                Command::Invalid(x.clone(), y.clone())
                            }
                        }

                        Command::Toggle(val) => Command::Increase(val.clone()),
                        Command::Invalid(x, y) => Command::JumpIfNotZero(x.clone(), y.clone()),
                    };

                    *cmd = replace_cmd;
                }

                1
            }
            Command::Invalid(_, _) => 1,
        }
    }
}

fn fetch_commands() -> Vec<Command> {
    read_text_from_file("16", "23")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_sent_value(mut commands: Vec<Command>, start: i32) -> i32 {
    let mut index = 0i32;
    let mut reg_map = HashMap::new();
    reg_map.insert("a".to_owned(), start);

    while index < commands.len() as i32 {
        let cmd = commands[index as usize].clone();
        index += cmd.apply(&mut reg_map, index, &mut commands);
    }

    *reg_map.get("a").unwrap()
}

fn part_1() {
    let commands = fetch_commands();

    let value = get_sent_value(commands, 7);

    println!("part_1: value is {value}");
}

fn part_2() {
    let commands = fetch_commands();

    let value = get_sent_value(commands, 12);

    println!("part_2: value is {value}");
}

pub fn run() {
    part_1();
    part_2();
}
