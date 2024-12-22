use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[inline]
fn first_proc(origin: u64) -> u64 {
    let mut res = origin * 64;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn second_proc(origin: u64) -> u64 {
    let mut res = origin / 32;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn third_prco(origin: u64) -> u64 {
    let mut res = origin * 2048;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn mix(n1: u64, n2: u64) -> u64 {
    n1 ^ n2
}

#[inline]
fn prune(n: u64) -> u64 {
    n % 16777216
}

#[inline]
fn process_step(origin: u64) -> u64 {
    let s1 = first_proc(origin);
    let s2 = second_proc(s1);
    third_prco(s2)
}

fn proc_num(mut origin: u64) -> u64 {
    for _ in 0..2000 {
        origin = process_step(origin);
    }
    origin
}

fn calc_nums(input: &str) -> u64 {
    let nums = parse(input);
    nums.into_par_iter().map(proc_num).sum()
}

fn part_1(input: &'static str) {
    let res = calc_nums(input);
    println!("Part 1 answer is {res}");
}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "22").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    const INPUT: &str = "\
1
10
100
2024";

    #[test]
    fn test_solution() {
        let res = calc_nums(INPUT);
        assert_eq!(res, 37327623);
    }
}

