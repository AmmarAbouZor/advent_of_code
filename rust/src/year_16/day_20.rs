use crate::utls::read_text_from_file;

#[derive(Debug)]
struct SimpleRange {
    min: usize,
    max: usize,
}

impl From<&str> for SimpleRange {
    fn from(value: &str) -> Self {
        let (min, max) = value.split_once('-').unwrap();
        let min = min.parse().unwrap();
        let max = max.parse().unwrap();

        assert!(min <= max);

        SimpleRange { min, max }
    }
}

impl SimpleRange {
    fn contain(&self, num: usize) -> bool {
        num >= self.min && num <= self.max
    }
}

fn fetch_ranges() -> Vec<SimpleRange> {
    read_text_from_file("16", "20")
        .lines()
        .map(|line| line.into())
        .collect()
}

fn part_1() {
    let mut ranges = fetch_ranges();

    ranges.sort_by_key(|r| r.min);

    let ip = (ranges.first().unwrap().min..ranges.last().unwrap().max)
        .find(|num| ranges.iter().all(|rng| !rng.contain(*num)))
        .unwrap();

    println!("lowest-valued IP is {ip}")
}

fn part_2() {
    let ranges = fetch_ranges();

    let min_bound = ranges.iter().map(|r| r.min).min().unwrap();
    let max_bound = ranges.iter().map(|r| r.max).max().unwrap();

    let allowed_count = (min_bound..max_bound)
        .filter(|num| ranges.iter().all(|rng| !rng.contain(*num)))
        .count();

    println!("allowed IPs count is {allowed_count}");
}

pub fn run() {
    part_1();
    part_2();
}
