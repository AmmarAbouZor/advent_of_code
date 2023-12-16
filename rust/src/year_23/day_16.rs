use std::collections::HashSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    dir: Dir,
}

impl State {
    fn new(pos: Point, dir: Dir) -> Self {
        Self { pos, dir }
    }

    fn next_pos(&self) -> Point {
        match self.dir {
            Dir::North => Point::new(self.pos.row.wrapping_sub(1), self.pos.col),
            Dir::East => Point::new(self.pos.row, self.pos.col + 1),
            Dir::South => Point::new(self.pos.row + 1, self.pos.col),
            Dir::West => Point::new(self.pos.row, self.pos.col.wrapping_sub(1)),
        }
    }

    fn do_move(self, grid: &[&[u8]]) -> Vec<State> {
        let next_pos = self.next_pos();
        let next_byte = match grid.get(next_pos.row).and_then(|row| row.get(next_pos.col)) {
            Some(&b) => b,
            None => return Vec::new(),
        };

        match (next_byte, self.dir) {
            (b'.', dir) => vec![State::new(next_pos, dir)],
            (b'|', Dir::South | Dir::North) => vec![State::new(next_pos, self.dir)],
            (b'|', Dir::East | Dir::West) => vec![
                State::new(next_pos, Dir::North),
                State::new(next_pos, Dir::South),
            ],
            (b'-', Dir::East | Dir::West) => vec![State::new(next_pos, self.dir)],
            (b'-', Dir::North | Dir::South) => vec![
                State::new(next_pos, Dir::East),
                State::new(next_pos, Dir::West),
            ],
            (b'/', Dir::North) => vec![State::new(next_pos, Dir::East)],
            (b'/', Dir::East) => vec![State::new(next_pos, Dir::North)],
            (b'/', Dir::South) => vec![State::new(next_pos, Dir::West)],
            (b'/', Dir::West) => vec![State::new(next_pos, Dir::South)],
            (b'\\', Dir::North) => vec![State::new(next_pos, Dir::West)],
            (b'\\', Dir::East) => vec![State::new(next_pos, Dir::South)],
            (b'\\', Dir::South) => vec![State::new(next_pos, Dir::East)],
            (b'\\', Dir::West) => vec![State::new(next_pos, Dir::North)],
            (invalid, dir) => unreachable!("Invalid State: pos:'{invalid}', dir: '{dir:?}'"),
        }
    }
}

fn energized_sum(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let first_dir = match grid[0][0] {
        b'\\' | b'|' => Dir::South,
        _ => Dir::East,
    };

    let mut states = vec![State::new(Point::new(0, 0), first_dir)];

    let mut visited_pos = HashSet::new();
    let mut visited_state = HashSet::new();

    while let Some(state) = states.pop() {
        visited_pos.insert(state.pos);
        state.do_move(&grid).into_iter().for_each(|s| {
            if visited_state.insert(s) {
                states.push(s);
            }
        });
    }

    visited_pos.len()
}

fn part_1(input: &str) {
    let answer = energized_sum(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "16");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_solution() {
        assert_eq!(energized_sum(INPUT), 46);
    }
}

