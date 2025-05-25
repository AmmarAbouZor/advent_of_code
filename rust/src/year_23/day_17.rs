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

    fn get_next_dirs(&self, min: u8, max: u8) -> Vec<(Dir, u8)> {
        let steps = self.stright_steps;

        if steps < min {
            return vec![(self.dir, self.stright_steps + 1)];
        }

        match self.dir {
            Dir::North if steps < max => vec![
                (Dir::North, self.stright_steps + 1),
                (Dir::East, 1),
                (Dir::West, 1),
            ],
            Dir::North => vec![(Dir::East, 1), (Dir::West, 1)],

            Dir::East if steps < max => vec![
                (Dir::East, self.stright_steps + 1),
                (Dir::North, 1),
                (Dir::South, 1),
            ],
            Dir::East => vec![(Dir::North, 1), (Dir::South, 1)],

            Dir::South if steps < max => vec![
                (Dir::South, self.stright_steps + 1),
                (Dir::East, 1),
                (Dir::West, 1),
            ],
            Dir::South => vec![(Dir::East, 1), (Dir::West, 1)],

            Dir::West if steps < max => vec![
                (Dir::West, self.stright_steps + 1),
                (Dir::North, 1),
                (Dir::South, 1),
            ],
            Dir::West => vec![(Dir::North, 1), (Dir::South, 1)],
        }
    }

    // States isn't checked if the grid contains them
    fn get_next_states(&self, min: u8, max: u8) -> Vec<State> {
        let dirs = self.get_next_dirs(min, max);
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

fn calc_min_heat(input: &str, min: u8, max: u8) -> usize {
    let grid = parse_input(input);

    let mut states_map: HashMap<State, usize> = HashMap::new();

    let starts = vec![
        State::new(Point::new(0, 0), Dir::East, 0),
        State::new(Point::new(0, 0), Dir::South, 0),
    ];

    let mut queue = BinaryHeap::new();

    for start in starts {
        states_map.insert(start.clone(), 0);
        queue.push((Reverse(0_usize), start));
    }

    let mut visited = HashSet::new();

    while let Some((_, curr_s)) = queue.pop() {
        if !visited.insert(curr_s.clone()) {
            continue;
        }
        let next_states = curr_s.get_next_states(min, max);

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
        .filter(|(state, _)| {
            state.pos.row == grid.len() - 1
                && state.pos.col == grid[0].len() - 1
                && state.stright_steps >= min
        })
        .map(|(_, heat)| heat)
        .min()
        .unwrap()
}

fn part_1(input: &str) {
    let answer = calc_min_heat(input, 0, 3);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_min_heat(input, 4, 10);

    println!("Part 2 answer is {answer}");
}

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

    const INPUT_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_solution() {
        assert_eq!(calc_min_heat(INPUT, 0, 3), 102);
        assert_eq!(calc_min_heat(INPUT, 4, 10), 94);
        assert_eq!(calc_min_heat(INPUT_2, 4, 10), 71);
    }
}
