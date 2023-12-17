use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    pos: Point,
    dir: Dir,
    stright_steps: u8,
}

impl State {
    fn new(pos: Point, dir: Dir, stright_steps: u8) -> Self {
        Self {
            pos,
            dir,
            stright_steps,
        }
    }

    fn get_next_dirs(&self) -> Vec<(Dir, u8)> {
        match (self.dir, self.stright_steps) {
            (Dir::North, 1..=2) => vec![
                (Dir::North, self.stright_steps + 1),
                (Dir::East, 1),
                (Dir::West, 1),
            ],
            (Dir::North, _) => vec![(Dir::East, 1), (Dir::West, 1)],

            (Dir::East, 1..=2) => vec![
                (Dir::East, self.stright_steps + 1),
                (Dir::North, 1),
                (Dir::South, 1),
            ],
            (Dir::East, _) => vec![(Dir::North, 1), (Dir::South, 1)],

            (Dir::South, 1..=2) => vec![
                (Dir::South, self.stright_steps + 1),
                (Dir::East, 1),
                (Dir::West, 1),
            ],
            (Dir::South, _) => vec![(Dir::East, 1), (Dir::West, 1)],

            (Dir::West, 1..=2) => vec![
                (Dir::West, self.stright_steps + 1),
                (Dir::North, 1),
                (Dir::South, 1),
            ],
            (Dir::West, _) => vec![(Dir::North, 1), (Dir::South, 1)],
        }
    }

    // States isn't checked if the grid contains them
    fn get_next_states(&self) -> Vec<State> {
        let dirs = self.get_next_dirs();
        dirs.into_iter()
            .map(|(dir, steps)| {
                let next_pos = match dir {
                    Dir::North => Point::new(self.pos.row.wrapping_sub(1), self.pos.col),
                    Dir::East => Point::new(self.pos.row, self.pos.col + 1),
                    Dir::South => Point::new(self.pos.row + 1, self.pos.col),
                    Dir::West => Point::new(self.pos.row, self.pos.col.wrapping_sub(1)),
                };

                State::new(next_pos, dir, steps)
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn calc_min_heat(input: &str) -> usize {
    let grid = parse_input(input);

    let mut states_map: HashMap<State, usize> = HashMap::new();

    let start = State::new(Point::new(0, 0), Dir::East, 0);

    states_map.insert(start.clone(), 0);

    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0_usize), start));

    let mut visited = HashSet::new();

    while let Some((_, curr_s)) = queue.pop() {
        if !visited.insert(curr_s.clone()) {
            continue;
        }
        let next_states = curr_s.get_next_states();

        for next_s in next_states {
            if let Some(&heat) = grid
                .get(next_s.pos.row)
                .and_then(|row| row.get(next_s.pos.col))
            {
                let mut total_heat = states_map.get(&curr_s).unwrap() + heat;
                states_map
                    .entry(next_s.clone())
                    .and_modify(|h| {
                        total_heat = total_heat.min(*h);
                        *h = total_heat;
                    })
                    .or_insert(total_heat);

                queue.push((Reverse(total_heat), next_s));
            }
        }
    }

    states_map
        .into_iter()
        .filter(|(state, _)| state.pos.row == grid.len() - 1 && state.pos.col == grid[0].len() - 1)
        .map(|(_, heat)| heat)
        .min()
        .unwrap()
}

fn part_1(input: &str) {
    let answer = calc_min_heat(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "17");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_solution() {
        assert_eq!(calc_min_heat(INPUT), 102);
    }
}

