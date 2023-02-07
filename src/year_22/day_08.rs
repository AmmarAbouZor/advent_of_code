use std::{num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;

        let cells = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(RADIX).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        Ok(Grid { cells })
    }
}

impl Grid {
    fn count_trees_in_sight(&self) -> usize {
        let width = self.cells.first().unwrap().len();
        let height = self.cells.len();
        let mut count = height * 2 + width * 2 - 4;

        for i in 1..width - 1 {
            for j in 1..height - 1 {
                let current = self.cells[j][i];
                let seen = (0..i)
                    .map(|index| self.cells[j][index])
                    .all(|cell| cell < current)
                    || (i + 1..width)
                        .map(|index| self.cells[j][index])
                        .all(|cell| cell < current)
                    || (0..j)
                        .map(|index| self.cells[index][i])
                        .all(|cell| cell < current)
                    || (j + 1..height)
                        .map(|index| self.cells[index][i])
                        .all(|cell| cell < current);

                if seen {
                    count += 1;
                }
            }
        }

        count
    }

    fn calc_highest_score(&self) -> usize {
        let width = self.cells.first().unwrap().len();
        let height = self.cells.len();
        let mut max = 0;

        for i in 1..width - 1 {
            for j in 1..height - 1 {
                max = max.max(self.calc_cell_score(j, i))
            }
        }

        max
    }

    fn calc_cell_score(&self, row: usize, col: usize) -> usize {
        let width = self.cells.first().unwrap().len();
        let height = self.cells.len();

        let current = self.cells[row][col];
        let mut left_score = (0..col)
            .rev()
            .map(|index| self.cells[row][index])
            .take_while(|c| *c < current)
            .count();

        if left_score < col {
            left_score += 1;
        }

        let mut right_score = (col + 1..width)
            .map(|index| self.cells[row][index])
            .take_while(|c| *c < current)
            .count();

        if right_score < width - col - 1 {
            right_score += 1;
        }

        let mut up_score = (0..row)
            .rev()
            .map(|index| self.cells[index][col])
            .take_while(|c| *c < current)
            .count();
        if up_score < row {
            up_score += 1;
        }

        let mut down_score = (row + 1..height)
            .map(|index| self.cells[index][col])
            .take_while(|c| *c < current)
            .count();

        if down_score < height - row - 1 {
            down_score += 1;
        }

        left_score * right_score * up_score * down_score
    }
}

fn part_1() {
    let input = read_text_from_file("22", "08");
    let grid: Grid = input.parse().unwrap();

    let answer = grid.count_trees_in_sight();

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "08");
    let grid: Grid = input.parse().unwrap();

    let score = grid.calc_highest_score();
    println!("Part 2 answer is {score}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        let grid: Grid = INPUT.parse().unwrap();
        assert_eq!(grid.count_trees_in_sight(), 21);
    }

    #[test]
    fn test_part_2() {
        let grid: Grid = INPUT.parse().unwrap();
        assert_eq!(grid.calc_highest_score(), 8);
    }
}
