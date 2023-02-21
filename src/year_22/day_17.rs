use std::collections::HashSet;

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

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
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
    fn apply_move(&mut self, dir: Dir, rocks: &HashSet<Point>, count: usize) -> bool {
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

        if count > 2 && clone.iter().any(|p| rocks.contains(&p)) {
            return dir != Dir::Down;
        }

        self.points = clone;
        return true;
    }

    fn set_start_pos(&mut self, shape: Shape, max_y: isize) {
        assert!(self.points.is_empty());
        match shape {
            Shape::Minus => {
                let height = max_y + 4;
                for i in 0..4 {
                    self.points.push(Point::new(i as isize + 2, height));
                }
                assert_eq!(self.points.len(), 4);
            }
            Shape::Plus => {
                self.points.push(Point::new(3, max_y + 6));
                let middle_hight = max_y + 5;
                for i in 1..4 {
                    self.points.push(Point::new(i as isize + 1, middle_hight));
                }
                self.points.push(Point::new(3, max_y + 4));
                assert_eq!(self.points.len(), 5);
            }
            Shape::ReversedL => {
                self.points.push(Point::new(4, max_y + 6));
                self.points.push(Point::new(4, max_y + 5));
                for i in 2..=4 {
                    self.points.push(Point::new(i as isize, max_y + 4));
                }
                assert_eq!(self.points.len(), 5);
            }
            Shape::IShape => {
                for i in 0..4 {
                    self.points
                        .push(Point::new(2, max_y + (3 - i as isize) + 4));
                }
                assert_eq!(self.points.len(), 4);
            }
            Shape::BigPoint => {
                for i in 0..2 {
                    self.points.push(Point::new(i as isize + 2, max_y + 5));
                }
                for i in 2..4 {
                    self.points.push(Point::new(i as isize, max_y + 4));
                }
                assert_eq!(self.points.len(), 4);
            }
        }
    }
}

struct Game {
    tetris_gen: Box<dyn Iterator<Item = Shape>>,
    dirs_gen: Box<dyn Iterator<Item = Dir>>,
    rocks: HashSet<Point>,
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

        let mut rocks = HashSet::new();

        (0..7).for_each(|i| {
            rocks.insert(Point::new(i, 0));
        });

        Game {
            tetris_gen,
            dirs_gen,
            rocks,
        }
    }

    fn simulate(&mut self, target: usize) -> isize {
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
                assert!(self.rocks.insert(p));
            });
        }

        self.rocks.iter().map(|p| p.y).max().unwrap()
    }

    fn simulate_2(&mut self) -> usize {
        let num = 40 * 5;
        for i in 0..num {
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
                assert!(self.rocks.insert(p));
            });
        }

        let mut max_y = self.rocks.iter().map(|p| p.y).max().unwrap();

        max_y as usize * 1000000000000 / num
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

    let answer = game.simulate(1000000000000);

    println!("Part_2 answer is {answer}");
}

pub fn run() {
    part_1();
    // part_2();
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
    #[ignore]
    fn test_part_2() {
        let mut game = Game::new(INPUT);
        assert_eq!(game.simulate_2(), 1514285714288);
    }
}
