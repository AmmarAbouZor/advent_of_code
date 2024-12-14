use std::{collections::HashSet, io};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Robot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl From<&str> for Robot {
    fn from(line: &str) -> Self {
        let (mut pos, mut vel) = line.split_once(' ').unwrap();
        pos = pos.trim_start_matches("p=");
        let (x, y) = pos.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        vel = vel.trim_start_matches("v=");
        let (dx, dy) = vel.split_once(',').unwrap();
        let dx = dx.parse().unwrap();
        let dy = dy.parse().unwrap();

        Self { x, y, dx, dy }
    }
}

impl Robot {
    fn mov(&mut self, max_x: isize, max_y: isize) {
        self.x = add_wrapping(self.x, self.dx, max_x);
        self.y = add_wrapping(self.y, self.dy, max_y);
    }
}

#[inline]
fn add_wrapping(val: isize, dv: isize, max: isize) -> isize {
    let mut ans = val + dv;
    if ans.is_negative() {
        ans += max;
    } else if ans >= max {
        ans %= max;
    }

    ans
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect()
}

fn calc_pos_count(input: &str, max_x: isize, max_y: isize) -> usize {
    let mut robots = parse(input);
    robots.par_iter_mut().for_each(|r| {
        for _ in 0..100 {
            r.mov(max_x, max_y);
        }
    });

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    use std::cmp::Ordering as Ord;

    for r in robots {
        match (r.x.cmp(&mid_x), r.y.cmp(&mid_y)) {
            (Ord::Less, Ord::Less) => q1 += 1,
            (Ord::Less, Ord::Equal) => {}
            (Ord::Less, Ord::Greater) => q2 += 1,
            (Ord::Equal, Ord::Less) => {}
            (Ord::Equal, Ord::Equal) => {}
            (Ord::Equal, Ord::Greater) => {}
            (Ord::Greater, Ord::Less) => q3 += 1,
            (Ord::Greater, Ord::Equal) => {}
            (Ord::Greater, Ord::Greater) => q4 += 1,
        }
    }

    q1 * q2 * q3 * q4
}

fn part_1(input: &'static str) {
    let ans = calc_pos_count(input, 101, 103);
    println!("Part 1 answer is {ans}");
}

// Print the steps where there are not overlapping in the robots.
fn print_unique_steps(input: &str, max_x: isize, max_y: isize) {
    let mut robots = parse(input);
    let mut sec = 0;
    let mut buffer = String::new();
    loop {
        robots.iter_mut().for_each(|r| r.mov(max_x, max_y));
        sec += 1;
        let set: HashSet<(isize, isize)> = HashSet::from_iter(robots.iter().map(|r| (r.x, r.y)));
        if set.len() != robots.len() {
            continue;
        }
        for x in 0..max_x {
            for y in 0..max_y {
                if set.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }

        println!("{sec}");
        io::stdin().read_line(&mut buffer).unwrap();
    }
}

fn part_2(input: &'static str) {
    print_unique_steps(input, 101, 103);
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "14").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_solution() {
        let ans = calc_pos_count(INPUT, 11, 7);
        assert_eq!(ans, 12);
    }
}
