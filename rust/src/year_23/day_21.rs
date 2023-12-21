use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<&'static [u8]>,
    start_pos: Point,
}

impl From<&'static str> for Grid {
    fn from(input: &'static str) -> Self {
        let cells: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let start_pos = cells
            .iter()
            .enumerate()
            .find_map(|(r, line)| {
                line.iter()
                    .position(|b| *b == b'S')
                    .map(|col| Point::new(r, col))
            })
            .unwrap();

        Self { cells, start_pos }
    }
}

impl Grid {
    fn get_possible_moves(&self, point: &Point) -> Vec<Point> {
        let moves = vec![
            Point::new(point.row.wrapping_sub(1), point.col),
            Point::new(point.row + 1, point.col),
            Point::new(point.row, point.col.wrapping_sub(1)),
            Point::new(point.row, point.col + 1),
        ];

        moves
            .into_iter()
            .filter(|p| {
                self.get_content(p)
                    .is_some_and(|&b| matches!(b, b'.' | b'S'))
            })
            .collect()
    }

    fn get_content(&self, point: &Point) -> Option<&u8> {
        self.cells.get(point.row).and_then(|row| row.get(point.col))
    }

    fn calc_max_plots(&mut self, max_steps: usize) -> usize {
        let mut plots_set = HashSet::new();

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), self.start_pos));

        while let Some((Reverse(mut steps), point)) = queue.pop() {
            if steps > max_steps {
                continue;
            }

            if !plots_set.insert((point, steps)) {
                continue;
            }

            steps += 1;
            for possible_point in self.get_possible_moves(&point) {
                queue.push((Reverse(steps), possible_point));
            }
        }

        plots_set
            .iter()
            .filter(|(_, count)| *count == max_steps)
            .count()
    }
}

fn calc_max_plots(input: &'static str, max_steps: usize) -> usize {
    let mut grid = Grid::from(input);

    grid.calc_max_plots(max_steps)
}

fn part_1(input: &'static str) {
    let answer = calc_max_plots(input, 64);

    println!("Part 1 answer is {answer}");
}

#[allow(unused)]
fn part_2(input: &'static str) {}

pub fn run() {
    let input = read_text_from_file("23", "21").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_solution() {
        assert_eq!(calc_max_plots(INPUT, 6), 16);
    }
}

