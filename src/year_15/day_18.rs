#![allow(warnings, unused)]

use crate::utls::read_lines_from_file;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    On,
    Off,
}

impl From<char> for State {
    fn from(ch: char) -> Self {
        match ch {
            '#' => State::On,
            '.' => State::Off,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Clone)]
struct LightsGrid {
    cells: Vec<Vec<State>>,
}

impl LightsGrid {
    fn new(cells: Vec<Vec<State>>) -> Self {
        Self { cells }
    }

    fn get_cell(&self, row: usize, col: usize) -> State {
        self.cells[row][col]
    }

    fn get_cell_or_off(&self, row: i32, col: i32) -> State {
        // casting negative int will give usize::Max - int
        let row = row as usize;
        let col = col as usize;

        if row < self.cells.len() && col < self.cells[0].len() {
            self.get_cell(row, col)
        } else {
            State::Off
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, value: State) {
        self.cells[row][col] = value;
    }

    fn get_on_lights_count(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|&cell| *cell == State::On)
            .count()
    }

    fn get_neighbors_on_count(&self, row: usize, col: usize) -> u8 {
        let row = row as i32;
        let col = col as i32;

        let mut on_count = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if r == row && c == col {
                    continue;
                }
                if self.get_cell_or_off(r, c) == State::On {
                    on_count += 1;
                }
            }
        }

        on_count
    }

    fn do_step(&mut self) {
        let clone = self.clone();
        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                let on_neighbors = clone.get_neighbors_on_count(row, col);
                match clone.get_cell(row, col) {
                    State::On => {
                        if on_neighbors != 2 && on_neighbors != 3 {
                            self.set_cell(row, col, State::Off);
                        }
                    }
                    State::Off => {
                        if on_neighbors == 3 {
                            self.set_cell(row, col, State::On);
                        }
                    }
                }
            }
        }
    }

    fn do_step_part_2(&mut self) {
        let last = self.cells.len() - 1;
        self.set_cell(0, 0, State::On);
        self.set_cell(0, last, State::On);
        self.set_cell(last, 0, State::On);
        self.set_cell(last, last, State::On);

        let clone = self.clone();
        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                let last_row = self.cells.len() - 1;
                let last_col = self.cells[0].len() - 1;
                if (row == 0 || row == last_row) && (col == 0 || col == last_col) {
                    continue;
                }

                let on_neighbors = clone.get_neighbors_on_count(row, col);
                match clone.get_cell(row, col) {
                    State::On => {
                        if on_neighbors != 2 && on_neighbors != 3 {
                            self.set_cell(row, col, State::Off);
                        }
                    }
                    State::Off => {
                        if on_neighbors == 3 {
                            self.set_cell(row, col, State::On);
                        }
                    }
                }
            }
        }
    }
}

fn fetch_input() -> LightsGrid {
    let cells: Vec<Vec<State>> = read_lines_from_file(r"src/year_15/day_18.txt")
        .into_iter()
        .map(|line| line.chars().map(|ch| ch.into()).collect::<Vec<State>>())
        .collect();
    LightsGrid::new(cells)
}

fn part_1() {
    let mut input = fetch_input();
    for _ in 0..100 {
        input.do_step();
    }

    let on_count = input.get_on_lights_count();
    println!("part one answer is {on_count}");
}
fn part_2() {
    let mut input = fetch_input();
    for _ in 0..100 {
        input.do_step_part_2();
    }

    let on_count = input.get_on_lights_count();

    println!("part two answer is {on_count}");
}

pub fn run() {
    part_1();
    part_2();
}
#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> LightsGrid {
        let input = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let cells = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.into()).collect::<Vec<State>>())
            .collect();
        LightsGrid::new(cells)
    }

    #[test]
    fn test_part_one() {
        let mut input = get_test_input();
        for _ in 0..4 {
            input.do_step();
        }
        let on_count = input.get_on_lights_count();

        assert_eq!(on_count, 4);
    }

    #[test]
    fn test_part_two() {
        let mut input = get_test_input();
        for _ in 0..5 {
            input.do_step_part_2();
        }
        let on_count = input.get_on_lights_count();

        assert_eq!(on_count, 17);
    }
}
