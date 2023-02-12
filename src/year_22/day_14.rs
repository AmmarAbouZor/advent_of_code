use std::collections::HashSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn get_below(&self) -> Point {
        Point {
            y: self.y + 1,
            ..*self
        }
    }

    fn get_diag_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn get_diag_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
struct Path {
    nodes: Vec<Point>,
}

impl Path {
    fn get_all_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for window in self.nodes.windows(2) {
            if window[0].x == window[1].x {
                let x = window[0].x;
                let min_y = window[0].y.min(window[1].y);
                let max_y = window[0].y.max(window[1].y);
                (min_y..=max_y).for_each(|y| {
                    points.push(Point::new(x, y));
                });
            } else {
                assert_eq!(window[0].y, window[1].y);
                let y = window[0].y;
                let min_x = window[0].x.min(window[1].x);
                let max_x = window[0].x.max(window[1].x);
                (min_x..=max_x).for_each(|x| {
                    points.push(Point::new(x, y));
                });
            }
        }

        points
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let nodes = value
            .split(" -> ")
            .map(|point| point.split_once(',').unwrap())
            .map(|(x, y)| {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();

                Point { x, y }
            })
            .collect();

        Path { nodes }
    }
}

fn calc_rest_units(input: &str) -> usize {
    let paths: Vec<Path> = input.lines().map(Path::from).collect();

    let max_y: i32 = paths
        .iter()
        .flat_map(|path| path.nodes.iter().map(|p| p.y))
        .max()
        .unwrap();

    let mut points_hash: HashSet<Point> = paths
        .into_iter()
        .flat_map(|path| path.get_all_points())
        .collect();

    let mut count_rest = 0;
    let mut sand_point = Point::new(500, 0);

    while sand_point.y <= max_y {
        let below = sand_point.get_below();
        if !points_hash.contains(&below) {
            sand_point = below;
            continue;
        }

        let diag_left = sand_point.get_diag_left();
        if !points_hash.contains(&diag_left) {
            sand_point = diag_left;
            continue;
        }

        let diag_right = sand_point.get_diag_right();
        if !points_hash.contains(&diag_right) {
            sand_point = diag_right;
            continue;
        }

        points_hash.insert(sand_point);
        sand_point = Point::new(500, 0);
        count_rest += 1;
    }

    count_rest
}

fn part_1() {
    let input = read_text_from_file("22", "14");

    let answer = calc_rest_units(&input);

    println!("Part 1 answer is {answer}");
}

fn calc_rest_sand_with_floor(input: &str) -> usize {
    let paths: Vec<Path> = input.lines().map(Path::from).collect();

    let max_y: i32 = paths
        .iter()
        .flat_map(|path| path.nodes.iter().map(|p| p.y))
        .max()
        .unwrap();

    let floor = max_y + 2;

    let mut points_hash: HashSet<Point> = paths
        .into_iter()
        .flat_map(|path| path.get_all_points())
        .collect();

    let mut count_rest = 0;
    let mut sand_point = Point::new(500, 0);

    let first_two_rows = [Point::new(500, 0), Point::new(499, 1), Point::new(501, 1)];
    while first_two_rows.iter().any(|p| !points_hash.contains(p)) {
        let below = sand_point.get_below();
        if below.y < floor && !points_hash.contains(&below) {
            sand_point = below;
            continue;
        }

        let diag_left = sand_point.get_diag_left();
        if diag_left.y < floor && !points_hash.contains(&diag_left) {
            sand_point = diag_left;
            continue;
        }

        let diag_right = sand_point.get_diag_right();
        if diag_right.y < floor && !points_hash.contains(&diag_right) {
            sand_point = diag_right;
            continue;
        }

        points_hash.insert(sand_point);
        sand_point = Point::new(500, 0);
        count_rest += 1;
    }

    count_rest
}

fn part_2() {
    let input = read_text_from_file("22", "14");

    let answer = calc_rest_sand_with_floor(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_rest_units(INPUT), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_rest_sand_with_floor(INPUT), 93);
    }
}
