use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<i16>>,
    start: Point,
    end: Point,
}

impl Grid {
    fn get_height(&self, point: &Point) -> i16 {
        assert!(!point.row.is_negative());
        assert!(!point.col.is_negative());

        self.cells[point.row as usize][point.col as usize]
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = Point::default();
        let mut end = Point::default();
        let cells = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        if ch.is_lowercase() {
                            ch as i16 - 'a' as i16
                        } else {
                            match ch {
                                'S' => {
                                    start.row = row as i16;
                                    start.col = col as i16;
                                    0
                                }
                                'E' => {
                                    end.row = row as i16;
                                    end.col = col as i16;
                                    'z' as i16 - 'a' as i16
                                }
                                _ => unreachable!("invalid input"),
                            }
                        }
                    })
                    .collect::<Vec<i16>>()
            })
            .collect();

        Ok(Grid { cells, start, end })
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    row: i16,
    col: i16,
}

impl Point {
    fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }

    fn get_valid_moves(&self, grid: &Grid, visited: &HashSet<Point>) -> Vec<Point> {
        let possible_point = [
            Point::new(self.row - 1, self.col),
            Point::new(self.row, self.col - 1),
            Point::new(self.row + 1, self.col),
            Point::new(self.row, self.col + 1),
        ];

        let height = grid.get_height(self);

        possible_point
            .into_iter()
            .filter(|point| {
                !point.row.is_negative()
                    && !point.col.is_negative()
                    && point.row < grid.cells.len() as i16
                    && point.col < grid.cells.first().unwrap().len() as i16
                    && !visited.contains(point)
                    && grid.get_height(point) - height <= 1
            })
            .collect()
    }
}

fn find_shortest_path(grid: &Grid, start: Point) -> usize {
    let mut states = VecDeque::new();
    let mut visited = HashSet::new();

    let mut distances = Vec::new();

    states.push_front((0, start));

    while let Some((distance, point)) = states.pop_back() {
        if point == grid.end {
            distances.push(distance);
        } else {
            point
                .get_valid_moves(grid, &visited)
                .into_iter()
                .for_each(|next_point| {
                    states.push_front((distance + 1, next_point));
                    visited.insert(next_point);
                });
        }
    }

    distances.into_iter().min().unwrap_or(usize::MAX)
}

fn find_path_from_start(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    find_shortest_path(&grid, grid.start)
}

fn find_path_from_any_a(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    grid.cells
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &val)| val == 0)
                .map(move |(col_index, _)| Point::new(row_index as i16, col_index as i16))
        })
        .map(|p| find_shortest_path(&grid, p))
        .min()
        .unwrap()
}

fn part_1() {
    let input = read_text_from_file("22", "12");
    let answer = find_path_from_start(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "12");
    let answer = find_path_from_any_a(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(find_path_from_start(INPUT), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(find_path_from_any_a(INPUT), 29);
    }
}
