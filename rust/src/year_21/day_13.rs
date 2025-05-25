use std::collections::HashSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Point { x, y }
    }
}

#[derive(Debug)]
struct Paper {
    cells: Option<HashSet<Point>>,
}

impl From<&str> for Paper {
    fn from(input: &str) -> Self {
        let cells = input.lines().map(Point::from).collect();

        Paper { cells: Some(cells) }
    }
}

impl Paper {
    pub fn get_count(&self) -> usize {
        self.cells.as_ref().unwrap().len()
    }

    pub fn apply_fold(&mut self, fold: Fold) {
        match fold {
            Fold::X(x) => self.fold_x(x),
            Fold::Y(y) => self.fold_y(y),
        }
    }

    fn fold_x(&mut self, x: usize) {
        let cells: HashSet<Point> = self
            .cells
            .take()
            .unwrap()
            .into_iter()
            .map(|mut p| {
                if p.x > x {
                    let diff = p.x - x;
                    p.x = x - diff;
                }

                p
            })
            .collect();

        self.cells = Some(cells);
    }

    fn fold_y(&mut self, y: usize) {
        let cells: HashSet<Point> = self
            .cells
            .take()
            .unwrap()
            .into_iter()
            .map(|mut p| {
                if p.y > y {
                    let diff = p.y - y;
                    p.y = y - diff;
                }

                p
            })
            .collect();

        self.cells = Some(cells);
    }

    fn print(&self) -> String {
        let mut chars = vec![vec!['.'; 40]; 6];

        for p in self.cells.as_ref().unwrap() {
            chars[p.y][p.x] = '#';
        }

        let lines: Vec<String> = chars
            .into_iter()
            .map(|chs| chs.into_iter().collect())
            .collect();

        lines.join("\n")
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<&str> for Fold {
    fn from(line: &str) -> Self {
        let instruction = line.split_whitespace().last().unwrap();

        match instruction.split_once('=') {
            Some(("x", num)) => Fold::X(num.parse().unwrap()),
            Some(("y", num)) => Fold::Y(num.parse().unwrap()),
            _ => unreachable!("Invalid input"),
        }
    }
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let (cells, instructions) = input.split_once("\n\n").unwrap();
    let paper = cells.into();
    let folds = instructions.lines().map(Fold::from).collect();

    (paper, folds)
}

fn get_count_after_one_fold(input: &str) -> usize {
    let (mut paper, folds) = parse_input(input);

    paper.apply_fold(folds.into_iter().next().unwrap());

    paper.get_count()
}

fn process_and_print(input: &str) {
    let (mut paper, folds) = parse_input(input);

    for fold in folds {
        paper.apply_fold(fold);
    }

    println!("{}", paper.print());
}

fn part_1() {
    let input = read_text_from_file("21", "13");
    let answer = get_count_after_one_fold(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "13");
    process_and_print(&input);
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part_1() {
        process_and_print(INPUT);
        assert_eq!(get_count_after_one_fold(INPUT), 17);
    }
}
