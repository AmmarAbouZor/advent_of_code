use std::collections::HashMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once(" -> ").unwrap();

        Segment {
            start: start.into(),
            end: end.into(),
        }
    }
}

impl Segment {
    #[inline]
    fn is_horz_ver(&self) -> bool {
        self.is_horz() || self.is_ver()
    }
    #[inline]
    fn is_horz(&self) -> bool {
        self.start.x == self.end.x
    }
    #[inline]
    fn is_ver(&self) -> bool {
        self.start.y == self.end.y
    }

    fn get_points_horz_ver(&self) -> Vec<Point> {
        if self.is_horz() {
            self.get_horz_points()
        } else if self.is_ver() {
            self.get_vert_points()
        } else {
            panic!("segment is not horizontal nor vertical");
        }
    }

    fn get_horz_points(&self) -> Vec<Point> {
        (self.start.y.min(self.end.y)..=self.start.y.max(self.end.y))
            .map(|y| Point::new(self.start.x, y))
            .collect()
    }

    fn get_vert_points(&self) -> Vec<Point> {
        (self.start.x.min(self.end.x)..=self.start.x.max(self.end.x))
            .map(|x| Point::new(x, self.start.y))
            .collect()
    }

    fn is_diagonal(&self) -> bool {
        let x_diff = self.start.x.abs_diff(self.end.x);
        let y_diff = self.start.y.abs_diff(self.end.y);

        x_diff == y_diff
    }

    fn get_diagonal_points(&self) -> Vec<Point> {
        let diff = self.start.x.abs_diff(self.end.x);

        let x_direction = if self.start.x < self.end.x { 1 } else { -1 };
        let y_directoin = if self.start.y < self.end.y { 1 } else { -1 };

        (0..=diff)
            .map(|d| {
                Point::new(
                    (self.start.x as isize + (d as isize * x_direction)) as u32,
                    (self.start.y as isize + (d as isize * y_directoin)) as u32,
                )
            })
            .collect()
    }

    fn get_hor_ver_diag(&self) -> Vec<Point> {
        if self.is_horz_ver() {
            self.get_points_horz_ver()
        } else if self.is_diagonal() {
            self.get_diagonal_points()
        } else {
            panic!("Segment is not horizontal nor vertical nor diagonal")
        }
    }
}

fn get_overlap_count(input: &str) -> usize {
    let mut map = HashMap::new();

    input
        .lines()
        .map(Segment::from)
        .filter(|segment| segment.is_horz_ver())
        .flat_map(|segment| segment.get_points_horz_ver())
        .for_each(|point| {
            map.entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    map.values().filter(|&&count| count >= 2).count()
}

fn part_1() {
    let input = read_text_from_file("21", "05");
    let answer = get_overlap_count(&input);

    println!("Part 1 answer is {answer}");
}

fn get_overlap_count_diag(input: &str) -> usize {
    let mut map = HashMap::new();

    input
        .lines()
        .map(Segment::from)
        .filter(|segment| segment.is_horz_ver() || segment.is_diagonal())
        .flat_map(|segment| segment.get_hor_ver_diag())
        .for_each(|point| {
            map.entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    map.values().filter(|&&count| count >= 2).count()
}

fn part_2() {
    let input = read_text_from_file("21", "05");
    let answer = get_overlap_count_diag(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part_1() {
        assert_eq!(get_overlap_count(INPUT), 5)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_overlap_count_diag(INPUT), 12)
    }
}
