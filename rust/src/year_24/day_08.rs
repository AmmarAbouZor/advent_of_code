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

fn antenna_antinodes(positions: &[Pos]) -> Vec<Pos> {
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

fn unique_antinodes(input: &str) -> usize {
    let grid = parse(input);
    let antennas = get_antennas(&grid);

    antennas
        .iter()
        // .take(1)
        .flat_map(|(_name, poses)| {
            let antinodes = antenna_antinodes(poses);
            antinodes
        })
        .filter(|antinode| {
            antinode.row >= 0
                && antinode.col >= 0
                && antinode.row < grid.len() as i32
                && antinode.col < grid[0].len() as i32
        })
        .unique()
        .count()
}

fn part_1(input: &'static str) {
    let ans = unique_antinodes(input);
    println!("Part 1 answer is {ans}")
}

fn part_2(input: &'static str) {}

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
        let count = unique_antinodes(INPUT);
        assert_eq!(count, 14);
    }
}

