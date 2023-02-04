use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Rng {
    start: usize,
    end: usize,
}

impl From<&str> for Rng {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Rng { start, end }
    }
}

impl Rng {
    fn fully_contain(&self, other: &Rng) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn contain(&self, num: usize) -> bool {
        num >= self.start && num <= self.end
    }
}

#[derive(Debug)]
struct Pair {
    left: Rng,
    right: Rng,
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(',').unwrap();
        let left = left.into();
        let right = right.into();

        Pair { left, right }
    }
}

impl Pair {
    fn fully_contained(&self) -> bool {
        self.left.fully_contain(&self.right) || self.right.fully_contain(&self.left)
    }

    fn overlap(&self) -> bool {
        self.left.contain(self.right.start)
            || self.left.contain(self.right.end)
            || self.right.contain(self.left.start)
            || self.right.contain(self.left.end)
    }
}

fn part_1() {
    let count = read_text_from_file("22", "04")
        .lines()
        .map(|line| Pair::from(line))
        .filter(|pair| pair.fully_contained())
        .count();

    println!("Part 1 answer is {count}");
}

fn part_2() {
    let count = read_text_from_file("22", "04")
        .lines()
        .map(|line| Pair::from(line))
        .filter(|pair| pair.overlap())
        .count();

    println!("Part 2 answer is {count}");
}

pub fn run() {
    part_1();
    part_2();
}
