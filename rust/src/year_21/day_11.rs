use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use crate::utls::read_text_from_file;

struct Octopus {
    cells: [[u8; 10]; 10],
    flash_count: usize,
}

impl From<&str> for Octopus {
    fn from(value: &str) -> Self {
        let mut cells = [[0; 10]; 10];
        for (i, row) in value.lines().enumerate() {
            for (j, num) in row.chars().map(|ch| ch.to_digit(10).unwrap()).enumerate() {
                cells[i][j] = num as u8;
            }
        }
        Octopus {
            cells,
            flash_count: 0,
        }
    }
}

impl Octopus {
    fn apply_step(&mut self) -> usize {
        let mut flash_que = VecDeque::new();

        let mut flash_set = HashSet::new();

        // Increase by one and fill the flash queue and set
        for row in 0..10 {
            for col in 0..10 {
                self.cells[row][col] += 1;
                if self.cells[row][col] == 10 {
                    self.cells[row][col] = 0;
                    flash_que.push_back((row, col));
                    flash_set.insert((row, col));
                }
            }
        }

        // Loop and flash
        while let Some((row, col)) = flash_que.pop_front() {
            self.flash_count += 1;

            let row_start = row.saturating_sub(1);
            let row_end = (row + 1).clamp(0, 9);
            let col_start = col.saturating_sub(1);
            let col_end = (col + 1).clamp(0, 9);

            for r in row_start..=row_end {
                for c in col_start..=col_end {
                    if flash_set.contains(&(r, c)) {
                        continue;
                    }

                    self.cells[r][c] += 1;
                    if self.cells[r][c] == 10 {
                        self.cells[r][c] = 0;
                        flash_que.push_back((r, c));
                        flash_set.insert((r, c));
                    }
                }
            }
        }

        flash_set.len()
    }
}

fn calc_total_flashes(input: &str, steps: usize) -> usize {
    let mut octopus = Octopus::from(input);
    for _ in 0..steps {
        octopus.apply_step();
    }

    octopus.flash_count
}

fn calc_first_sync(input: &str) -> usize {
    let mut octopus = Octopus::from(input);
    let mut count = 1;

    while octopus.apply_step() != 100 {
        count += 1;
    }

    count
}

fn part_1() {
    let input = read_text_from_file("21", "11");
    let answer = calc_total_flashes(&input, 100);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "11");
    let answer = calc_first_sync(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_octopus() {
        assert_eq!(calc_total_flashes(INPUT, 10), 204);
        assert_eq!(calc_total_flashes(INPUT, 100), 1656);
        assert_eq!(calc_first_sync(INPUT), 195);
    }
}

