use std::collections::{BinaryHeap, HashSet};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cells = value
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        Self { cells }
    }
}

impl Grid {
    fn calc_risk_level(&self) -> u32 {
        let mut sum = 0;

        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                if let Some(cell) = self.check_cell_low(row, col) {
                    sum += cell + 1;
                }
            }
        }

        sum
    }

    fn check_cell_low(&self, row: usize, col: usize) -> Option<u32> {
        let cell = self.cells[row][col];
        let up = self
            .cells
            .get(row.wrapping_sub(1))
            .map_or(u32::MAX, |r| *r.get(col).unwrap());

        if cell >= up {
            return None;
        }

        let down = self
            .cells
            .get(row + 1)
            .map_or(u32::MAX, |r| *r.get(col).unwrap());

        if cell >= down {
            return None;
        }

        let left = *self.cells[row]
            .get(col.wrapping_sub(1))
            .unwrap_or(&u32::MAX);
        if cell >= left {
            return None;
        }

        let right = *self.cells[row].get(col + 1).unwrap_or(&u32::MAX);
        if cell >= right {
            return None;
        }

        Some(cell)
    }

    fn calc_basians_sum(&self) -> usize {
        let mut basians_heap = BinaryHeap::new();

        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                if self.check_cell_low(row, col).is_none() {
                    continue;
                }

                let basian = self.calc_basian(row, col);
                basians_heap.push(basian);
            }
        }

        (0..3).map(|_| basians_heap.pop().unwrap()).product()
    }

    fn calc_basian(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        let mut visited_set = HashSet::new();
        let mut stack = vec![(row, col)];

        while let Some((r, c)) = stack.pop() {
            if !visited_set.insert((r, c)) {
                continue;
            }

            count += 1;

            let adjacens = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];

            adjacens
                .into_iter()
                .filter(|&(row, col)| {
                    self.cells
                        .get(row)
                        .map(|r| r.get(col).unwrap_or(&u32::MAX))
                        .is_some_and(|&val| val < 9)
                })
                .for_each(|(row, col)| {
                    stack.push((row, col));
                });
        }

        count
    }
}

fn part_1() {
    let input = read_text_from_file("21", "09");
    let grid = Grid::from(input.as_str());
    let answer = grid.calc_risk_level();

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "09");
    let grid = Grid::from(input.as_str());
    let answer = grid.calc_basians_sum();

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_grid() {
        let grid = Grid::from(INPUT);
        assert_eq!(grid.calc_risk_level(), 15);
        assert_eq!(grid.calc_basians_sum(), 1134);
    }
}
