use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn is_inside(&self, rows_count: i32, cols_count: i32) -> bool {
        self.row >= 0 && self.col >= 00 && self.row < rows_count && self.col < cols_count
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<Pos>> {
    let mut antennas = HashMap::new();
    grid.iter().enumerate().for_each(|(row, chars)| {
        chars
            .iter()
            .enumerate()
            .filter(|(_, &char)| char != '.')
            .for_each(|(col, &char)| {
                let pos = Pos::new(row as i32, col as i32);
                antennas.entry(char).or_insert(Vec::new()).push(pos);
            });
    });

    antennas
}

fn antenna_antinodes_once(positions: &[Pos]) -> Vec<Pos> {
    let mut res = Vec::with_capacity(positions.len() * 2);
    for cur in positions {
        for other in positions {
            if cur == other {
                continue;
            }
            let dif_r = other.row - cur.row;
            let dif_c = other.col - cur.col;

            let row = cur.row - dif_r;
            let col = cur.col - dif_c;

            res.push(Pos::new(row, col));
        }
    }

    res
}

fn unique_antinodes_once(input: &str) -> usize {
    let grid = parse(input);
    let antennas = get_antennas(&grid);

    antennas
        .iter()
        .flat_map(|(_name, poses)| {
            let antinodes = antenna_antinodes_once(poses);
            antinodes
        })
        .filter(|antinode| antinode.is_inside(grid.len() as i32, grid[0].len() as i32))
        .unique()
        .count()
}

fn part_1(input: &'static str) {
    let ans = unique_antinodes_once(input);
    println!("Part 1 answer is {ans}")
}

fn antenna_antinodes_all(positions: &[Pos], rows: i32, cols: i32) -> Vec<Pos> {
    let mut res = Vec::with_capacity(positions.len() * 2);
    for cur in positions {
        for other in positions {
            if cur == other {
                continue;
            }
            let dif_r = other.row - cur.row;
            let dif_c = other.col - cur.col;

            for factor in 0.. {
                let row = cur.row - (factor * dif_r);
                let col = cur.col - (factor * dif_c);
                let pos = Pos::new(row, col);
                if pos.is_inside(rows, cols) {
                    res.push(pos);
                } else {
                    break;
                }
            }
        }
    }

    res
}

fn unique_antinodes_all(input: &str) -> usize {
    let grid = parse(input);
    let antennas = get_antennas(&grid);

    antennas
        .iter()
        .flat_map(|(_name, poses)| {
            let antinodes = antenna_antinodes_all(poses, grid.len() as i32, grid[0].len() as i32);
            antinodes
        })
        .unique()
        .count()
}

fn part_2(input: &'static str) {
    let ans = unique_antinodes_all(input);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "08").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_solution() {
        let count = unique_antinodes_once(INPUT);
        assert_eq!(count, 14);

        let all_count = unique_antinodes_all(INPUT);
        assert_eq!(all_count, 34);
    }
}

