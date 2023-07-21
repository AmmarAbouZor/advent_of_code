use std::collections::BTreeMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => unreachable!("Invalid input"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Default, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Minus,
    Plus,
    ReversedL,
    IShape,
    BigPoint,
}

#[derive(Debug, Default, Clone)]
struct Tetris {
    points: Vec<Point>,
}

impl Tetris {
    fn apply_move(&mut self, dir: Dir, rocks: &[Point], count: usize) -> bool {
        let clone = match dir {
            Dir::Left => {
                let min_x = self.points.iter().map(|p| p.x).min().unwrap();
                if min_x > 0 {
                    let mut clone = self.points.clone();
                    clone.iter_mut().for_each(|p| p.x -= 1);

                    clone
                } else {
                    return true;
                }
            }
            Dir::Right => {
                let max_x = self.points.iter().map(|p| p.x).max().unwrap();
                if max_x < 6 {
                    let mut clone = self.points.clone();
                    clone.iter_mut().for_each(|p| p.x += 1);
                    clone
                } else {
                    return true;
                }
            }
            Dir::Down => {
                let mut clone = self.points.clone();
                clone.iter_mut().for_each(|p| p.y -= 1);
                clone
            }
        };

        if count > 2 && clone.iter().any(|p| rocks.contains(p)) {
            return dir != Dir::Down;
        }

        self.points = clone;

        true
    }

    fn set_start_pos(&mut self, shape: Shape, max_y: usize) {
        assert!(self.points.is_empty());
        match shape {
            Shape::Minus => {
                let height = max_y + 4;
                for i in 0..4 {
                    self.points.push(Point::new(i + 2, height));
                }
            }
            Shape::Plus => {
                self.points.push(Point::new(3, max_y + 6));
                let middle_hight = max_y + 5;
                for i in 1..4 {
                    self.points.push(Point::new(i + 1, middle_hight));
                }
                self.points.push(Point::new(3, max_y + 4));
            }
            Shape::ReversedL => {
                self.points.push(Point::new(4, max_y + 6));
                self.points.push(Point::new(4, max_y + 5));
                for i in 2..=4 {
                    self.points.push(Point::new(i, max_y + 4));
                }
            }
            Shape::IShape => {
                for i in 0..4 {
                    self.points.push(Point::new(2, max_y + (3 - i) + 4));
                }
            }
            Shape::BigPoint => {
                for i in 0..2 {
                    self.points.push(Point::new(i + 2, max_y + 5));
                }
                for i in 2..4 {
                    self.points.push(Point::new(i, max_y + 4));
                }
            }
        }
    }
}

struct Game {
    tetris_gen: Box<dyn Iterator<Item = Shape>>,
    dirs_gen: Box<dyn Iterator<Item = Dir>>,
    rocks: Vec<Point>,
}

impl Game {
    fn new(input: &str) -> Game {
        let shapes: Vec<Shape> = vec![
            Shape::Minus,
            Shape::Plus,
            Shape::ReversedL,
            Shape::IShape,
            Shape::BigPoint,
        ];

        let tetris_gen = Box::new(shapes.into_iter().cycle());

        let dirs_gen = Box::new(
            input
                .chars()
                .map(Dir::from)
                .collect::<Vec<Dir>>()
                .into_iter()
                .cycle(),
        );

        let mut rocks = Vec::new();

        (0..7).for_each(|i| {
            rocks.push(Point::new(i, 0));
        });

        Game {
            tetris_gen,
            dirs_gen,
            rocks,
        }
    }

    fn simulate(&mut self, target: usize) -> usize {
        for _ in 0..target {
            let shape = self.tetris_gen.next().unwrap();
            let max_y = self.rocks.iter().map(|p| p.y).max().unwrap();
            let mut tetris = Tetris::default();
            tetris.set_start_pos(shape, max_y);

            let mut count = 0;
            loop {
                let dir = self.dirs_gen.next().unwrap();
                if !tetris.apply_move(dir, &self.rocks, count) {
                    break;
                }
                if !tetris.apply_move(Dir::Down, &self.rocks, count) {
                    break;
                }

                count += 1;
            }

            tetris.points.into_iter().for_each(|p| {
                self.rocks.push(p);
            });
        }

        self.rocks.iter().map(|p| p.y).max().unwrap()
    }

    fn simulate_2(&mut self, target: usize) -> usize {
        let mut last_cycle_limit = 0;
        let mut last_i = 0;

        let mut first_appearence = None;
        let mut cycle_diff = None;
        let mut height_map = BTreeMap::new();

        let mut found = false;

        let mut i = 0;

        while i < target {
            let shape = self.tetris_gen.next().unwrap();
            let max_y = self.rocks.iter().map(|p| p.y).max().unwrap();
            let mut tetris = Tetris::default();
            tetris.set_start_pos(shape, max_y);

            let mut count = 0;
            loop {
                let dir = self.dirs_gen.next().unwrap();
                if !tetris.apply_move(dir, &self.rocks, count) {
                    break;
                }
                if !tetris.apply_move(Dir::Down, &self.rocks, count) {
                    break;
                }

                count += 1;
            }

            tetris.points.into_iter().for_each(|p| {
                self.rocks.push(p);
            });

            let height = self.rocks.iter().map(|p| p.y).max().unwrap();

            height_map.insert(i, height);

            if found {
                i += 1;
                continue;
            }

            let cycle_limit = height.saturating_sub(15);

            let mut rest = Vec::new();

            let check_range: Vec<usize> = self
                .rocks
                .iter()
                .filter(|p| p.y > last_cycle_limit)
                .filter(|p| {
                    if p.y > cycle_limit {
                        true
                    } else {
                        rest.push(p.x);
                        false
                    }
                })
                .map(|p| p.x)
                .collect();

            if check_range.is_empty() || rest.len() < check_range.len() {
                i += 1;
                continue;
            }

            if rest
                .windows(check_range.len())
                .any(|sl| sl == check_range.as_slice())
            {
                last_cycle_limit = cycle_limit;

                if let Some(first_appearence_index) = first_appearence {
                    cycle_diff = Some(i - last_i);

                    let start_cycle = first_appearence_index - cycle_diff.unwrap();

                    let cycles_count = (target - start_cycle) / cycle_diff.unwrap();

                    i = start_cycle + cycles_count * cycle_diff.unwrap();
                    found = true;
                    continue;
                } else {
                    first_appearence = Some(i - last_i);
                }

                last_i = i;
            }

            i += 1;
        }

        let start_cycle = first_appearence.unwrap() - cycle_diff.unwrap();

        let height_cycle_diff = *height_map
            .get(&(cycle_diff.unwrap() + first_appearence.unwrap()))
            .unwrap()
            - *height_map.get(&first_appearence.unwrap()).unwrap();

        let cycles_count = (target - start_cycle) / cycle_diff.unwrap();

        let last_height = height_map.get(&(target - 1)).unwrap();

        last_height + ((cycles_count - 2) * height_cycle_diff) - 1
    }
}

fn part_1() {
    let input = read_text_from_file("22", "17");

    let mut game = Game::new(&input);

    let answer = game.simulate(2022);

    println!("Part_1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "17");

    let mut game = Game::new(&input);

    let answer = game.simulate_2(1000000000000);

    println!("Part_2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_1() {
        let mut game = Game::new(INPUT);
        assert_eq!(game.simulate(2022), 3068);
    }
    #[test]
    fn test_part_2() {
        let mut game = Game::new(INPUT);
        assert_eq!(game.simulate_2(1000000000000), 1514285714288);
    }
}
