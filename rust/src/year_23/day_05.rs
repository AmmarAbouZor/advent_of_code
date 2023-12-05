use std::{isize, ops::RangeInclusive};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Maps {
    SeedSoil,
    SoilFert,
    FertWater,
    WaterLight,
    LightTemp,
    TempHumi,
    HumiLocation,
}

const MAPS_LENGTH: usize = Maps::HumiLocation as usize + 1;

#[derive(Debug)]
struct RangeDiff {
    src_rng: RangeInclusive<isize>,
    diff: isize,
}

impl RangeDiff {
    fn new(src_rng: RangeInclusive<isize>, diff: isize) -> Self {
        Self { src_rng, diff }
    }
    fn try_solve(&self, num: isize) -> Option<isize> {
        if self.src_rng.contains(&num) {
            Some(num + self.diff)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<isize>,
    maps: Vec<Vec<RangeDiff>>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let mut seeds = parts.next().unwrap();
        seeds = seeds.strip_prefix("seeds: ").unwrap();
        let seeds: Vec<isize> = seeds
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let maps = parts
            .map(|range_txt| {
                // first line is irrelevant
                let (_, range_txt) = range_txt.split_once('\n').unwrap();

                range_txt
                    .lines()
                    .map(|line| {
                        let parts: Vec<isize> = line
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect();
                        let source_rng = RangeInclusive::new(parts[1], parts[1] + parts[2]);
                        let diff = parts[0] - parts[1];

                        RangeDiff::new(source_rng, diff)
                    })
                    .collect()
            })
            .collect();

        Almanac { seeds, maps }
    }
}

impl Almanac {
    fn find_location(&self, init_num: isize) -> isize {
        self.maps.iter().fold(init_num, |num, map| {
            map.iter()
                .find_map(|rng_diff| rng_diff.try_solve(num))
                .unwrap_or(num)
        })
    }

    fn find_min_location(&self) -> isize {
        self.seeds
            .iter()
            .map(|seed| self.find_location(*seed))
            .min()
            .unwrap()
    }
}

fn find_lowes(input: &str) -> isize {
    let almanac = Almanac::from(input);
    almanac.find_min_location()
}

fn part_1(input: &str) {
    let answer_1 = find_lowes(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "05");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_solution() {
        assert_eq!(find_lowes(INPUT), 35);
    }
}

