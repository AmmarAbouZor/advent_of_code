use std::ops::RangeInclusive;

use crate::utls::read_text_from_file;

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

    fn generate_ranges(&self) -> Vec<RangeInclusive<isize>> {
        self.seeds.chunks(2).fold(Vec::new(), |mut ranges, rng| {
            ranges.push(rng[0]..=rng[0] + rng[1] - 1);
            ranges
        })
    }

    fn find_min_location_ranges(&self) -> isize {
        let start_ranges = self.generate_ranges();

        let final_ranges = self.maps.iter().fold(start_ranges, |ranges, map| {
            ranges
                .into_iter()
                .flat_map(|rng| self.solve_ranges(map, rng))
                .collect()
        });

        final_ranges.iter().map(|rng| *rng.start()).min().unwrap()
    }

    fn solve_ranges(
        &self,
        rng_diffs: &[RangeDiff],
        rng: RangeInclusive<isize>,
    ) -> Vec<RangeInclusive<isize>> {
        let mut ranges = Vec::new();

        // This will be used to find out which chunks of the range aren't mapped
        let mut temp_ranges = Vec::new();

        for rng_diff in rng_diffs {
            let src_rng = &rng_diff.src_rng;
            match (src_rng.contains(rng.start()), src_rng.contains(rng.end())) {
                // Inside
                (true, true) => {
                    ranges.push(*rng.start() + rng_diff.diff..=*rng.end() + rng_diff.diff);
                    temp_ranges.push(*rng.start()..=*rng.end());
                }
                // Start inside. End outside
                (true, false) => {
                    ranges.push(*rng.start() + rng_diff.diff..=*src_rng.end() + rng_diff.diff);
                    temp_ranges.push(*rng.start()..=*src_rng.end());
                }
                // Start outside. End inside
                (false, true) => {
                    ranges.push(*src_rng.start() + rng_diff.diff..=*rng.end() + rng_diff.diff);
                    temp_ranges.push(*src_rng.start()..=*rng.end());
                }
                (false, false) => {
                    let overlapp = rng.start() < src_rng.start() && rng.end() > src_rng.end();
                    if overlapp {
                        ranges.push(
                            *src_rng.start() + rng_diff.diff..=*src_rng.end() + rng_diff.diff,
                        );
                        temp_ranges.push(*src_rng.start()..=*src_rng.end());
                    }
                }
            }
        }

        // Fill out the unmapped chunks
        temp_ranges.sort_unstable_by_key(|rng| *rng.start());

        if ranges.is_empty() {
            ranges.push(rng.clone());
        } else {
            let mut bound_to_check = *rng.start();
            for tmp_rng in temp_ranges {
                if bound_to_check < *tmp_rng.start() {
                    ranges.push(bound_to_check..=*tmp_rng.start() - 1);
                }
                bound_to_check = *tmp_rng.end();
            }
        }

        ranges
    }
}

fn find_lowes(input: &str) -> isize {
    let almanac = Almanac::from(input);
    almanac.find_min_location()
}

fn find_lowes_ranges(input: &str) -> isize {
    let almanac = Almanac::from(input);
    almanac.find_min_location_ranges()
}

fn part_1(input: &str) {
    let answer_1 = find_lowes(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {
    let answer_2 = find_lowes_ranges(input);

    println!("Part 2 answer is {answer_2}");
}

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
        assert_eq!(find_lowes_ranges(INPUT), 46);
    }
}
