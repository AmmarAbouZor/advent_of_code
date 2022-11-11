#![allow(warnings, unused)]

use std::{
    cmp, fs,
    io::{self, BufRead},
};

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2x3x4";

    #[test]
    fn test_present_from() {
        let present = Present::from(INPUT);

        assert_eq!(present.length, 2);
        assert_eq!(present.width, 3);
        assert_eq!(present.height, 4);
    }

    #[test]
    fn test_present_surface() {
        let present = Present::from(INPUT);
        assert_eq!(present.calc_surface_area(), 52);

        let present = Present::from("1x1x10");
        assert_eq!(present.calc_surface_area(), 42);
    }

    #[test]
    fn test_present_smallest_area() {
        let present = Present::from(INPUT);
        assert_eq!(present.calc_smallest_area(), 6);

        let present = Present::from("1x1x10");
        assert_eq!(present.calc_smallest_area(), 1);
    }

    #[test]
    fn test_present_all_paper() {
        let present = Present::from(INPUT);
        assert_eq!(present.calc_all_paper(), 58);

        let present = Present::from("1x1x10");
        assert_eq!(present.calc_all_paper(), 43);
    }

    #[test]
    fn test_present_ribbon() {
        let present = Present::from(INPUT);
        assert_eq!(present.calc_ribbon(), 34);

        let present = Present::from("1x1x10");
        assert_eq!(present.calc_ribbon(), 14);
    }
}

#[derive(Debug)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl From<&str> for Present {
    fn from(text: &str) -> Self {
        let dimensions: Vec<&str> = text.split('x').collect();

        assert_eq!(dimensions.len(), 3);

        let length = dimensions[0].parse().unwrap();
        let width = dimensions[1].parse().unwrap();
        let height = dimensions[2].parse().unwrap();

        Present {
            length,
            width,
            height,
        }
    }
}

impl Present {
    fn calc_surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn get_smallest_dimensions(&self) -> [u32; 2] {
        if self.length <= self.width {
            [self.length, cmp::min(self.width, self.height)]
        } else {
            [self.width, cmp::min(self.length, self.height)]
        }
    }

    fn calc_smallest_area(&self) -> u32 {
        self.get_smallest_dimensions().iter().product()
    }

    fn calc_all_paper(&self) -> u32 {
        self.calc_surface_area() + self.calc_smallest_area()
    }

    fn calc_ribbon(&self) -> u32 {
        let bowl = self.length * self.width * self.height;

        let shortest_distance: u32 = self.get_smallest_dimensions().iter().sum();

        bowl + shortest_distance * 2
    }
}

pub fn run() {
    let file = fs::File::open(r"src/year_15/day_2.txt").unwrap();

    let presents: Vec<Present> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Present::from(line.as_str()))
        .collect();

    let sum_paper: u32 = presents
        .iter()
        .map(|present| present.calc_all_paper())
        .sum();

    println!("total square feet is: {sum_paper}");

    let sum_ribbon: u32 = presents.iter().map(|present| present.calc_ribbon()).sum();

    println!("total ribbon is: {sum_ribbon}");
}
