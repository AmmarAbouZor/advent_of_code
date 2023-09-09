use std::ops::RangeInclusive;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct TargetRange {
    x_rng: RangeInclusive<i32>,
    y_rng: RangeInclusive<i32>,
}

impl From<&str> for TargetRange {
    fn from(input: &str) -> Self {
        let (_, range_part) = input.split_once(": ").unwrap();
        let (x_part, y_part) = range_part.split_once(", ").unwrap();
        let x_part = x_part.strip_prefix("x=").unwrap();
        let (x_start, x_end) = x_part.split_once("..").unwrap();
        let x_rng = RangeInclusive::new(x_start.parse().unwrap(), x_end.parse().unwrap());
        let y_part = y_part.strip_prefix("y=").unwrap();
        let (y_start, y_end) = y_part.split_once("..").unwrap();
        let y_rng = RangeInclusive::new(y_start.parse().unwrap(), y_end.parse().unwrap());

        TargetRange { x_rng, y_rng }
    }
}

impl TargetRange {
    fn contains(&self, point: &Point) -> bool {
        self.x_rng.contains(&point.x) && self.y_rng.contains(&point.y)
    }

    fn ckeck_missed_range(&self, point: &Point) -> bool {
        point.x >= *self.x_rng.end() || point.y <= *self.y_rng.start()
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn find_highest_pos(input: &str) -> i32 {
    let target = TargetRange::from(input);

    dbg!(target);

    todo!()
}

fn part_1() {
    let input = read_text_from_file("21", "17");
    let answer = find_highest_pos(input.as_str());

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

    const INPUT: &str = r"target area: x=20..30, y=-10..-5";

    #[test]
    fn test_target_range() {
        let target = TargetRange::from(INPUT);

        let p_inside = Point::new(20, -7);
        assert!(target.contains(&p_inside));
        assert!(!target.ckeck_missed_range(&p_inside));

        let p_miss_x = Point::new(31, -7);
        assert!(!target.contains(&p_miss_x));
        assert!(target.ckeck_missed_range(&p_miss_x));

        let p_miss_y = Point::new(21, -11);
        assert!(!target.contains(&p_miss_y));
        assert!(target.ckeck_missed_range(&p_miss_y));

        let p_not_inside_not_missed = Point::new(15, 10);
        assert!(!target.contains(&p_not_inside_not_missed));
        assert!(!target.ckeck_missed_range(&p_not_inside_not_missed));
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(find_highest_pos(INPUT), 45);
    // }
}

