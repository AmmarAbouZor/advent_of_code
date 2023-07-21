use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Destination {
    Bot(usize),
    OutPut(usize),
}

#[derive(Debug)]
struct GiveCommand {
    bot: usize,
    low: Destination,
    high: Destination,
}

#[derive(Debug)]
struct AssignCommand {
    value: usize,
    bot: usize,
}

impl FromStr for AssignCommand {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.len() {
            6 => {
                let value = parts[1].parse()?;
                let bot = parts.last().unwrap().parse()?;

                Ok(AssignCommand { value, bot })
            }
            _ => panic!("invalid input"),
        }
    }
}

impl FromStr for GiveCommand {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let get_dest = |text: &str, id: usize| {
            if text == "bot" {
                Destination::Bot(id)
            } else {
                Destination::OutPut(id)
            }
        };
        match parts.len() {
            12 => {
                let bot = parts[1].parse()?;
                let low_id = parts[6].parse()?;
                let high_id = parts.last().unwrap().parse()?;
                let low = get_dest(parts[5], low_id);
                let high = get_dest(parts[10], high_id);

                Ok(GiveCommand { bot, low, high })
            }
            _ => panic!("invalid input"),
        }
    }
}

struct Bot {
    chips: Vec<usize>,
}

impl Bot {
    fn new() -> Self {
        let chips = vec![];
        Self { chips }
    }

    fn give_chips(&mut self) -> Option<(usize, usize)> {
        if self.chips.len() == 2 {
            self.chips.sort();
            let high = self.chips.pop().unwrap();
            let low = self.chips.pop().unwrap();

            Some((low, high))
        } else {
            None
        }
    }

    fn add_chip(&mut self, chip: usize) {
        self.chips.push(chip)
    }
}

struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, usize>,
}

impl Factory {
    fn new() -> Self {
        let bots = HashMap::new();
        let outputs = HashMap::new();

        Self { bots, outputs }
    }

    fn apply_assign(&mut self, cmd: &AssignCommand) {
        let bot = self.bots.entry(cmd.bot).or_insert_with(Bot::new);
        bot.add_chip(cmd.value);
    }

    fn apply_give(&mut self, cmd: &GiveCommand) -> Option<(usize, usize)> {
        let bot = self.bots.entry(cmd.bot).or_insert_with(Bot::new);
        if let Some((low, high)) = bot.give_chips() {
            match cmd.low {
                Destination::Bot(bot) => {
                    let bot = self.bots.entry(bot).or_insert_with(Bot::new);
                    bot.add_chip(low);
                }
                Destination::OutPut(output) => {
                    self.outputs.insert(output, low);
                }
            }

            match cmd.high {
                Destination::Bot(bot) => {
                    let bot = self.bots.entry(bot).or_insert_with(Bot::new);
                    bot.add_chip(high);
                }
                Destination::OutPut(output) => {
                    self.outputs.insert(output, high);
                }
            }

            Some((low, high))
        } else {
            None
        }
    }

    fn get_keys_multi(&self) -> Option<usize> {
        if let (Some(val_0), Some(val_1), Some(val_2)) = (
            self.outputs.get(&0),
            self.outputs.get(&1),
            self.outputs.get(&2),
        ) {
            Some(val_0 * val_1 * val_2)
        } else {
            None
        }
    }
}

fn parse_commands() -> (Vec<AssignCommand>, Vec<GiveCommand>) {
    let text = read_text_from_file("16", "10");
    let (ass, give): (Vec<_>, Vec<_>) = text.lines().partition(|line| line.starts_with("value"));

    let assign_cmd = ass.iter().map(|l| l.parse().unwrap()).collect();
    let give_cmd = give.iter().map(|l| l.parse().unwrap()).collect();

    (assign_cmd, give_cmd)
}

fn part_1() {
    let (assigned, mut give) = parse_commands();

    let mut factory = Factory::new();

    for cmd in assigned {
        factory.apply_assign(&cmd);
    }

    let mut found = false;
    while !found && give.is_empty() {
        let mut executed_cmds = vec![];
        for gv_cmd in give.iter() {
            if let Some((low, high)) = factory.apply_give(gv_cmd) {
                if low == 17 && high == 61 {
                    println!("bot num {} is the one", gv_cmd.bot);
                    found = true;
                    break;
                }
                executed_cmds.push(gv_cmd.bot);
            }
        }

        give.retain(|cmd| !executed_cmds.contains(&cmd.bot));
    }
}

fn part_2() {
    let (assigned, mut give) = parse_commands();

    let mut factory = Factory::new();

    for cmd in assigned {
        factory.apply_assign(&cmd);
    }

    while give.is_empty() {
        let mut executed_cmds = vec![];
        for gv_cmd in give.iter() {
            if let Some((_, _)) = factory.apply_give(gv_cmd) {
                executed_cmds.push(gv_cmd.bot);
            }
        }

        give.retain(|cmd| !executed_cmds.contains(&cmd.bot));

        if let Some(val) = factory.get_keys_multi() {
            println!("value of 0 1 2 is {val}");
            break;
        }
    }
}

pub fn run() {
    part_1();
    part_2();
}
