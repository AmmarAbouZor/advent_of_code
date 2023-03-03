use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    pos: Point,
    dir: Dir,
}

impl Blizzard {
    fn new(pos: Point, dir: Dir) -> Self {
        Self { pos, dir }
    }

    fn apply_move(&mut self, width: usize, height: usize) {
        match self.dir {
            Dir::Up => {
                if self.pos.row > 1 {
                    self.pos.row -= 1;
                } else {
                    self.pos.row = height - 2;
                }
            }
            Dir::Down => {
                if self.pos.row < height - 2 {
                    self.pos.row += 1;
                } else {
                    self.pos.row = 1;
                }
            }
            Dir::Left => {
                if self.pos.col > 1 {
                    self.pos.col -= 1;
                } else {
                    self.pos.col = width - 2;
                }
            }
            Dir::Right => {
                if self.pos.col < width - 2 {
                    self.pos.col += 1;
                } else {
                    self.pos.col = 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn fetch_blizzards(input: &str) -> (Vec<Blizzard>, usize, usize) {
    let lines: Vec<&str> = input.lines().collect();

    let blizzards = (1..lines.len() - 1)
        .flat_map(|row| {
            lines[1]
                .chars()
                .enumerate()
                .filter(|(_, ch)| !['#', '.'].contains(ch))
                .map(move |(col, ch)| Blizzard::new(Point::new(row, col), ch.into()))
        })
        .collect();

    (
        blizzards,
        lines.len(),
        lines.first().unwrap().chars().count(),
    )
}

#[derive(Debug, Clone)]
struct State {
    minutes: usize,
    current_pos: Point,
}
impl State {
    fn get_possibilities(&self, blizzards: &[Blizzard], width: usize, height: usize) -> Vec<Point> {
        let movements = [
            Point::new(self.current_pos.row, self.current_pos.col + 1),
            Point::new(self.current_pos.row, self.current_pos.col - 1),
            Point::new(self.current_pos.row + 1, self.current_pos.col),
            Point::new(self.current_pos.row - 1, self.current_pos.col),
        ];

        movements
            .into_iter()
            .filter(|p| {
                p.row > 0
                    && p.col > 0
                    && p.row < height - 1
                    && p.col < width - 1
                    && blizzards.iter().all(|bliz| bliz.pos != *p)
            })
            .collect()
    }
}

fn calc_min_minutes(input: &str) -> usize {
    let (mut blizzards, height, width) = fetch_blizzards(input);
    let target = Point::new(height - 2, width - 2);

    // dbg!(&blizzards);
    // dbg!(&height);
    // dbg!(&width);
    // dbg!(&start);
    // dbg!(&target);

    blizzards
        .iter_mut()
        .for_each(|b| b.apply_move(width, height));

    let mut bliz_map = BTreeMap::from([(1, blizzards)]);

    let pos = Point::new(1, 1);

    let one_move_state = State {
        minutes: 1,
        current_pos: pos,
    };

    let mut states = VecDeque::from([one_move_state]);
    let mut min_score = 25;

    while let Some(state) = states.pop_back() {
        // dbg!(&state.current_pos);
        if state.current_pos == target {
            min_score = min_score.min(state.minutes);
            dbg!(&min_score);
            continue;
        }

        // already past the best we have
        if state.minutes > min_score {
            continue;
        }

        if !bliz_map.contains_key(&(state.minutes + 1)) {
            let mut before = bliz_map.get(&(state.minutes)).unwrap().clone();

            before.iter_mut().for_each(|b| b.apply_move(width, height));

            bliz_map.insert(state.minutes + 1, before);
        }

        let blizzards = bliz_map.get(&(state.minutes + 1)).unwrap();
        let possibilities = state.get_possibilities(blizzards, width, height);

        //wait option
        let mut clone = state.clone();
        clone.minutes += 1;
        states.push_front(clone);

        for pos in possibilities {
            let mut clone = state.clone();
            clone.minutes += 1;
            clone.current_pos = pos;
            states.push_back(clone);
        }
    }

    min_score
}

fn part_1() {
    let input = read_text_from_file("22", "24");

    let answer = calc_min_minutes(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_min_minutes(INPUT), 18);
    }
}
