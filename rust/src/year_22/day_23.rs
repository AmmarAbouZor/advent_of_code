use std::collections::HashSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

#[derive(Debug, Clone)]
struct Elf {
    pos: Point,
    propose: Option<Point>,
}

impl Elf {
    fn new(pos: Point) -> Self {
        let propose = None;
        Self { pos, propose }
    }

    fn calc_moving_propose(&mut self, elves: &[Elf], dirs: &[Dir]) {
        assert!(self.propose.is_none());
        let north_free = elves
            .iter()
            .filter(|e| e.pos.y == self.pos.y + 1)
            .all(|e| !(self.pos.x - 1..=self.pos.x + 1).contains(&e.pos.x));

        let south_free = elves
            .iter()
            .filter(|e| e.pos.y == self.pos.y - 1)
            .all(|e| !(self.pos.x - 1..=self.pos.x + 1).contains(&e.pos.x));

        let west_free = elves
            .iter()
            .filter(|e| e.pos.x == self.pos.x - 1)
            .all(|e| !(self.pos.y - 1..=self.pos.y + 1).contains(&e.pos.y));

        let east_free = elves
            .iter()
            .filter(|e| e.pos.x == self.pos.x + 1)
            .all(|e| !(self.pos.y - 1..=self.pos.y + 1).contains(&e.pos.y));

        let all_free = north_free && south_free && west_free && east_free;

        if all_free {
            self.propose = None;
        } else {
            for dir in dirs.iter() {
                match dir {
                    Dir::North if north_free => {
                        self.propose = Some(Point::new(self.pos.x, self.pos.y + 1));
                        break;
                    }
                    Dir::South if south_free => {
                        self.propose = Some(Point::new(self.pos.x, self.pos.y - 1));
                        break;
                    }
                    Dir::West if west_free => {
                        self.propose = Some(Point::new(self.pos.x - 1, self.pos.y));
                        break;
                    }
                    Dir::East if east_free => {
                        self.propose = Some(Point::new(self.pos.x + 1, self.pos.y));
                        break;
                    }
                    _ => {}
                };
            }
        }
    }

    fn move_if_possible(&mut self, proposes: &[Point], duplicates: &mut HashSet<Point>) -> bool {
        if let Some(propose) = self.propose.take() {
            if !duplicates.contains(&propose) {
                let propose_count = proposes
                    .iter()
                    .filter(|p| p.x == propose.x && p.y == propose.y)
                    .count();
                assert!(propose_count > 0);

                if propose_count > 1 {
                    duplicates.insert(propose);
                    return false;
                } else {
                    self.pos = propose;
                    return true;
                }
            }
        }

        false
    }
}

fn fetch_input(input: &str) -> Vec<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| Elf::new(Point::new(x as isize, -(y as isize))))
        })
        .collect()
}

fn calc_empty_tiles(input: &str) -> usize {
    let mut elves = fetch_input(input);

    let mut prop_dir = vec![Dir::North, Dir::South, Dir::West, Dir::East];

    for _ in 0..10 {
        let clones = elves.clone();
        elves
            .iter_mut()
            .for_each(|elf| elf.calc_moving_propose(&clones, &prop_dir));

        let mut duplicates = HashSet::new();

        let proposes: Vec<Point> = elves.iter().flat_map(|elf| elf.propose).collect();

        elves.iter_mut().for_each(|elf| {
            elf.move_if_possible(&proposes, &mut duplicates);
        });

        prop_dir.rotate_left(1);
    }

    let max_x = elves.iter().map(|elf| elf.pos.x).max().unwrap();
    let min_x = elves.iter().map(|elf| elf.pos.x).min().unwrap();
    let max_y = elves.iter().map(|elf| elf.pos.y).max().unwrap();
    let min_y = elves.iter().map(|elf| elf.pos.y).min().unwrap();

    let width = max_y.abs_diff(min_y);
    let height = max_x.abs_diff(min_x);

    (width + 1) * (height + 1) - elves.len()
}

fn find_first_no_move(input: &str) -> usize {
    let mut elves = fetch_input(input);

    let mut prop_dir = vec![Dir::North, Dir::South, Dir::West, Dir::East];

    for round in 1.. {
        let clones = elves.clone();
        elves
            .iter_mut()
            .for_each(|elf| elf.calc_moving_propose(&clones, &prop_dir));

        let mut duplicates = HashSet::new();

        let proposes: Vec<Point> = elves.iter().flat_map(|elf| elf.propose).collect();

        if proposes.is_empty() {
            return round;
        }

        let moves = elves
            .iter_mut()
            .map(|elf| elf.move_if_possible(&proposes, &mut duplicates))
            .filter(|m| *m)
            .count();

        if moves == 0 {
            return round;
        }

        prop_dir.rotate_left(1);
    }

    unreachable!();
}

fn part_1() {
    let input = read_text_from_file("22", "23");

    let answer = calc_empty_tiles(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "23");

    let answer = find_first_no_move(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_empty_tiles(INPUT), 110);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(find_first_no_move(INPUT), 20);
    }
}
