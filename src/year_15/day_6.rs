#![allow(warnings, unused)]

use std::{
    fs,
    io::{self, BufRead},
};

use crate::utls::read_lines_from_file;

#[derive(Debug)]
struct LightsGrid<T> {
    cells: Vec<Vec<T>>,
}

impl<T: Copy> LightsGrid<T> {
    fn new(init_value: T) -> Self {
        let cells = vec![vec![init_value; 1000]; 1000];
        Self { cells }
    }

    fn get_cell(&self, row: usize, col: usize) -> T {
        self.cells[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: T) {
        self.cells[row][col] = value;
    }

    fn get_all_cells(&self) -> Vec<&T> {
        self.cells.iter().flatten().collect::<Vec<&T>>()
    }
}

struct Cell {
    row: usize,
    col: usize,
}

impl From<&str> for Cell {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(',').collect();

        let row: usize = parts[0].parse().unwrap();
        let col: usize = parts[1].parse().unwrap();

        Cell { row, col }
    }
}

struct Instruction {
    start: Cell,
    end: Cell,
    command: Command,
}

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

impl From<&str> for Instruction {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(' ').collect();
        match parts.len() {
            4 => Instruction {
                start: parts[1].into(),
                end: parts[3].into(),
                command: Command::Toggle,
            },
            5 if parts[1] == "on" => Instruction {
                start: parts[2].into(),
                end: parts[4].into(),
                command: Command::TurnOn,
            },
            5 => Instruction {
                start: parts[2].into(),
                end: parts[4].into(),
                command: Command::TurnOff,
            },
            _ => panic!("Invalid text"),
        }
    }
}

fn apply_inst_bools(grid: &mut LightsGrid<bool>, inst: &Instruction) {
    for row in inst.start.row..=inst.end.row {
        for col in inst.start.col..=inst.end.col {
            let value = match inst.command {
                Command::TurnOn => true,
                Command::TurnOff => false,
                Command::Toggle => !grid.get_cell(row, col),
            };
            grid.set_cell(row, col, value);
        }
    }
}

fn apply_inst_nums(grid: &mut LightsGrid<i32>, inst: &Instruction) {
    for row in inst.start.row..=inst.end.row {
        for col in inst.start.col..=inst.end.col {
            let value = match inst.command {
                Command::TurnOn => 1,
                Command::TurnOff => -1,
                Command::Toggle => 2,
            };
            let mut add_value = grid.get_cell(row, col) + value;
            if add_value < 0 {
                add_value = 0;
            }
            grid.set_cell(row, col, add_value);
        }
    }
}

fn run_bools() {
    let mut grid = LightsGrid::new(false);

    read_lines_from_file(r"src/year_15/day_6.txt")
        .into_iter()
        .for_each(|line| {
            let inst: Instruction = line.as_str().into();
            apply_inst_bools(&mut grid, &inst);
        });

    let on_lights = grid.get_all_cells().iter().filter(|&&&cell| cell).count();

    println!("number of lit lights are {on_lights}");
}

fn run_nums() {
    let mut grid = LightsGrid::new(0i32);

    read_lines_from_file(r"src/year_15/day_6.txt")
        .into_iter()
        .for_each(|line| {
            let inst: Instruction = line.as_str().into();
            apply_inst_nums(&mut grid, &inst);
        });

    let total_brightness: i32 = grid.get_all_cells().iter().map(|&&cell| cell).sum();

    println!("Total brightness is: {total_brightness}");
}

pub fn run() {
    run_bools();
    run_nums();
}
