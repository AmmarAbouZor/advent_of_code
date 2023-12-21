use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
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
                    .map(|col| Point::new(r as isize, col as isize))
            })
            .unwrap();

        Self { cells, start_pos }
    }
}

impl Grid {
    fn get_possible_moves(&self, point: &Point) -> Vec<Point> {
        let moves = vec![
            Point::new(point.row - 1, point.col),
            Point::new(point.row + 1, point.col),
            Point::new(point.row, point.col - 1),
            Point::new(point.row, point.col + 1),
        ];

        moves
            .into_iter()
            .filter(|p| {
                let b = self.get_content(p).unwrap();
                matches!(b, b'.' | b'S')
            })
            .collect()
    }

    fn get_content(&self, point: &Point) -> Option<&u8> {
        let rows_total = self.cells.len() as isize;
        let cols_total = self.cells[0].len() as isize;
        let mut row = point.row;
        let mut col = point.col;
        while row.is_negative() {
            row += rows_total;
        }
        while col.is_negative() {
            col += cols_total;
        }

        let row = (row % rows_total) as usize;
        let col = (col % cols_total) as usize;

        self.cells.get(row).and_then(|r| r.get(col))
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

fn part_2(input: &'static str) {
    // The grid is a square with a side length of 131
    // The target is exactly 26501365 steps = 202300 * 131 + (131 / 2)
    // This will only solve for the case where total total_steps = NUM * square_side_length + (square_side_length / 2)

    // Get enough data from the normal algorithm used in part 1
    let mut answer_samples: Vec<_> = (0..=2)
        .into_par_iter()
        .map(|i| {
            let mut grid = Grid::from(input);
            grid.calc_max_plots(65 + i * 131)
        })
        .collect();

    // There is no guarantee that rayon will return item in order, even though it must be always
    // the case here since each iteration will take significant more time than the previous one
    answer_samples.sort_unstable();

    // Get the rest of answers by extrapolating the last value from the answers until we reach
    // the target value
    while answer_samples.len() <= (26501365 - 65) / 131 {
        answer_samples.push(extrapolate(&answer_samples));
    }

    let answer = answer_samples.last().unwrap();

    println!("Part 2 answer is {answer}");
}

fn extrapolate(values: &[usize]) -> usize {
    let differences = values
        .windows(2)
        .map(|win| win[1] - win[0])
        .collect::<Vec<_>>();
    if differences.iter().all(|x| x == &0) {
        *values.iter().last().unwrap()
    } else {
        values.iter().last().unwrap() + extrapolate(&differences)
    }
}

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
        assert_eq!(calc_max_plots(INPUT, 50), 1594);
        assert_eq!(calc_max_plots(INPUT, 100), 6536);
    }
}
