use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply_step(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.y += 1,
            Dir::Down => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    dir: Dir,
    steps: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse()?;

        let dir = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => unreachable!("invalid input"),
        };

        Ok(Move { dir, steps })
    }
}

trait Rope {
    fn apply_move(&mut self, mov: &Move);
    fn get_visited_pos(&self) -> usize;
}

#[derive(Debug, Default)]
struct ShortRope {
    head: Point,
    tail: Point,
    visited_pos: BTreeSet<Point>,
}

impl ShortRope {
    fn move_tail(head: &Point, tail: &mut Point) {
        let diff_x = head.x - tail.x;
        let diff_y = head.y - tail.y;
        if diff_x.abs() == 2 {
            tail.x += diff_x / 2;
            if diff_y.abs() == 1 {
                tail.y += diff_y;
            }
        } else if diff_y.abs() == 2 {
            tail.y += diff_y / 2;
            if diff_x.abs() == 1 {
                tail.x += diff_x;
            }
        }
    }
}

impl Rope for ShortRope {
    fn apply_move(&mut self, mov: &Move) {
        for _ in 0..mov.steps {
            self.head.apply_step(&mov.dir);
            ShortRope::move_tail(&self.head, &mut self.tail);
            self.visited_pos.insert(self.tail);
        }
    }

    fn get_visited_pos(&self) -> usize {
        self.visited_pos.len()
    }
}

#[derive(Debug, Default)]
struct LongRope {
    knots: [Point; 10],
    visited_pos: BTreeSet<Point>,
}

impl Rope for LongRope {
    fn apply_move(&mut self, mov: &Move) {
        for _ in 0..mov.steps {
            self.knots[0].apply_step(&mov.dir);

            for index in 0..self.knots.len() - 1 {
                LongRope::move_tail(&mut self.knots[index..=index + 1]);
            }

            self.visited_pos.insert(*self.knots.last().unwrap());
        }
    }

    fn get_visited_pos(&self) -> usize {
        self.visited_pos.len()
    }
}

impl LongRope {
    fn move_tail(slice: &mut [Point]) {
        let diff_x = slice[0].x - slice[1].x;
        let diff_y = slice[0].y - slice[1].y;
        assert!(diff_x < 3);
        assert!(diff_y < 3);
        if diff_x.abs() == 2 {
            slice[1].x += diff_x / 2;
            if diff_y.abs() != 0 {
                slice[1].y += diff_y / diff_y.abs();
            }
        } else if diff_y.abs() == 2 {
            slice[1].y += diff_y / 2;
            if diff_x.abs() != 0 {
                slice[1].x += diff_x / diff_x.abs();
            }
        }
    }
}

fn calc_visited_pos<T: Rope>(rope: &mut T, input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|mov| rope.apply_move(&mov));

    rope.get_visited_pos()
}

fn part_1() {
    let input = read_text_from_file("22", "09");
    let mut rope = ShortRope::default();
    let answer = calc_visited_pos(&mut rope, &input);

    println!("Part 1 answer is {answer}");
}
fn part_2() {
    let input = read_text_from_file("22", "09");
    let mut rope = LongRope::default();
    let answer = calc_visited_pos(&mut rope, &input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        let mut rope = ShortRope::default();
        assert_eq!(calc_visited_pos(&mut rope, INPUT), 13);
    }

    const INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_2() {
        let mut rope = LongRope::default();
        assert_eq!(calc_visited_pos(&mut rope, INPUT_2), 36);
    }
}
