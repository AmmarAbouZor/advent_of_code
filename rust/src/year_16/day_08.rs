use std::{num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Command {
    Rect(u8, u8),
    RotRow(u8, u8),
    RotCol(u8, u8),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_ascii_whitespace().collect();
        match parts.len() {
            2 => {
                let (wide, tall) = parts[1].split_once('x').unwrap();
                let wide = wide.parse()?;
                let tall = tall.parse()?;
                Ok(Command::Rect(wide, tall))
            }
            _ => {
                let index = parts[2][2..].parse()?;
                let shift = parts[4].parse()?;
                if parts[1] == "row" {
                    Ok(Command::RotRow(index, shift))
                } else {
                    Ok(Command::RotCol(index, shift))
                }
            }
        }
    }
}

struct Screen {
    pixels: [[bool; 50]; 6],
}

impl Screen {
    fn new() -> Self {
        let pixels = [[false; 50]; 6];
        Self { pixels }
    }

    fn apply_command(&mut self, cmd: Command) {
        match cmd {
            Command::Rect(wide, tall) => {
                for row in 0..tall as usize {
                    for col in 0..wide as usize {
                        self.pixels[row][col] = true;
                    }
                }
            }
            Command::RotRow(index, shift) => {
                let shift = shift as usize;
                self.pixels[index as usize].rotate_right(shift);
            }
            Command::RotCol(index, shift) => {
                let mut cols: Vec<bool> =
                    (0..6).map(|row| self.pixels[row][index as usize]).collect();

                let shift = shift as usize;
                cols.rotate_right(shift);

                (0..6).for_each(|row| self.pixels[row][index as usize] = cols[row]);
            }
        }
    }

    fn get_on_pixels_count(&self) -> usize {
        self.pixels.iter().flatten().filter(|p| **p).count()
    }

    fn print(&self) {
        let bool_to_char = |b: &bool| match b {
            true => '#',
            false => '.',
        };

        for line in self.pixels.iter() {
            let line: String = line.iter().map(bool_to_char).collect();
            println!("{line}");
        }
    }
}

fn parse_input() -> Vec<Command> {
    read_text_from_file("16", "08")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn run() {
    let mut screen = Screen::new();

    for cmd in parse_input() {
        screen.apply_command(cmd);
    }

    println!("num of lit pixels: {}", screen.get_on_pixels_count());

    screen.print();
}
