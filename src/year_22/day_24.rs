use std::collections::BTreeSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    fn get_possible_poss(
        &self,
        bliz_set: &BTreeSet<Point>,
        width: usize,
        height: usize,
    ) -> Vec<Point> {
        let movements = [
            Point::new(self.row, self.col + 1),
            Point::new(self.row, self.col.saturating_sub(1)),
            Point::new(self.row + 1, self.col),
            Point::new(self.row.saturating_sub(1), self.col),
            Point::new(self.row, self.col),
        ];

        movements
            .into_iter()
            .filter(|p| {
                (p.row > 0 || (p.row == 0 && p.col == 1))
                    && p.col > 0
                    && (p.row < height - 1 || (p.row == height - 1 && p.col == width - 2))
                    && p.col < width - 1
                    && !bliz_set.contains(p)
            })
            .collect()
    }
}

fn fetch_blizzards(input: &str) -> (Vec<Blizzard>, usize, usize) {
    let lines: Vec<&str> = input.lines().collect();

    let blizzards = (1..lines.len() - 1)
        .flat_map(|row| {
            lines[row]
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

fn calc_min_minutes(input: &str) -> usize {
    let (mut blizzards, height, width) = fetch_blizzards(input);
    let target = Point::new(height - 1, width - 2);
    let mut current_poss = BTreeSet::from([Point::new(0, 1)]);

    for mins in 1.. {
        blizzards
            .iter_mut()
            .for_each(|b| b.apply_move(width, height));

        let bliz_set: BTreeSet<_> = blizzards.iter().map(|bl| bl.pos).collect();
        current_poss = current_poss
            .into_iter()
            .flat_map(|p| p.get_possible_poss(&bliz_set, width, height))
            .collect();

        if current_poss.contains(&target) {
            return mins;
        }

        assert!(!current_poss.is_empty());
    }

    unreachable!();
}

fn calc_min_minutes_repeated(input: &str) -> usize {
    let (mut blizzards, height, width) = fetch_blizzards(input);
    let end = Point::new(height - 1, width - 2);
    let start = Point::new(0, 1);

    let mut target = end;
    let mut current_poss = BTreeSet::from([start]);

    let mut count = 0;

    for mins in 1.. {
        blizzards
            .iter_mut()
            .for_each(|b| b.apply_move(width, height));

        let bliz_set: BTreeSet<_> = blizzards.iter().map(|bl| bl.pos).collect();
        current_poss = current_poss
            .into_iter()
            .flat_map(|p| p.get_possible_poss(&bliz_set, width, height))
            .collect();

        if current_poss.contains(&target) {
            match count {
                0 => {
                    target = start;
                    current_poss = BTreeSet::from([end]);
                    count += 1;
                }
                1 => {
                    target = end;
                    current_poss = BTreeSet::from([start]);
                    count += 1;
                }
                2 => {
                    return mins;
                }
                _ => unreachable!(),
            }
        }

        assert!(!current_poss.is_empty());
    }

    unreachable!();
}

fn part_1() {
    let input = read_text_from_file("22", "24");

    let answer = calc_min_minutes(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "24");

    let answer = calc_min_minutes_repeated(&input);

    println!("Part 2 answer is {answer}");
}

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
    #[test]
    fn test_part_2() {
        assert_eq!(calc_min_minutes_repeated(INPUT), 54);
    }
}
