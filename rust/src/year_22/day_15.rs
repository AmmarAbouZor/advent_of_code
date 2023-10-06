use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
struct Sensor {
    position: Point,
    beacon: Point,
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();

        let x = parts[2]
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse()
            .unwrap();
        let y = parts[3]
            .trim_start_matches("y=")
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let position = Point { x, y };

        let x = parts[8]
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse()
            .unwrap();
        let y = parts[9].trim_start_matches("y=").parse().unwrap();

        let beacon = Point { x, y };

        Sensor { position, beacon }
    }
}

impl Sensor {
    fn get_points_target_y(&self, target_y: isize) -> HashSet<Point> {
        let radius =
            (self.position.x - self.beacon.x).abs() + (self.position.y - self.beacon.y).abs();
        let mut coverage = HashSet::new();

        let diff = target_y - self.position.y;

        if diff.abs() <= radius && !(radius - diff.abs()).is_negative() {
            for x in 0..=radius - diff.abs() {
                coverage.insert(Point::new(self.position.x + x, self.position.y + diff));
                coverage.insert(Point::new(self.position.x - x, self.position.y + diff));
            }
        }

        coverage
    }

    fn get_converage_area(&self, max: isize) -> HashMap<isize, RangeInclusive<isize>> {
        let radius =
            (self.position.x - self.beacon.x).abs() + (self.position.y - self.beacon.y).abs();
        let mut ranges_map = HashMap::new();

        if self.position.x - radius <= max || self.position.y <= max {
            for y in 0..=radius {
                let x_bound = radius - y;

                if self.position.y + y <= max {
                    ranges_map.insert(
                        self.position.y + y,
                        self.position.x - x_bound..=self.position.x + x_bound,
                    );
                }

                if self.position.y - y <= max {
                    ranges_map.insert(
                        self.position.y - y,
                        self.position.x - x_bound..=self.position.x + x_bound,
                    );
                }
            }
        }

        ranges_map
    }
}

fn get_no_beacon_count(input: &str, target_y: isize) -> usize {
    let sensors = input.lines().map(Sensor::from);

    let beacons: HashSet<Point> = sensors
        .clone()
        .map(|s| s.beacon)
        .filter(|s| s.y == target_y)
        .collect();

    let total_points_withen_y: HashSet<Point> = sensors
        .flat_map(|sensor| sensor.get_points_target_y(target_y))
        .collect();

    total_points_withen_y
        .iter()
        .filter(|p| !beacons.contains(p))
        .count()
}

fn part_1() {
    let input = read_text_from_file("22", "15");

    let answer = get_no_beacon_count(&input, 2000000);

    println!("Part 1 answer is {answer}");
}

fn get_tuning_frequency(input: &str, max: isize) -> isize {
    let sensors = input.lines().map(Sensor::from);

    let mut ranges_map: HashMap<isize, Vec<RangeInclusive<isize>>> = HashMap::new();

    sensors
        .flat_map(|sensor| sensor.get_converage_area(max))
        .for_each(|(y, rng)| {
            let rngs = ranges_map.entry(y).or_default();
            rngs.push(rng);
        });

    for y in 0..=max {
        let rngs = ranges_map.get_mut(&y).unwrap();
        rngs.sort_by(|a, b| a.start().cmp(b.start()));

        let mut pos = 0;

        for rng in rngs {
            if *rng.start() > pos {
                return (pos + 1) * 4000000 + y;
            }

            pos = pos.max(*rng.end());
        }
    }

    unreachable!();
}

fn part_2() {
    let input = read_text_from_file("22", "15");

    let answer = get_tuning_frequency(&input, 4000000);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_1() {
        assert_eq!(get_no_beacon_count(INPUT, 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_tuning_frequency(INPUT, 20), 56000011);
    }
}
