use std::collections::{HashMap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[inline]
fn first_proc(origin: i64) -> i64 {
    let mut res = origin * 64;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn second_proc(origin: i64) -> i64 {
    let mut res = origin / 32;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn third_prco(origin: i64) -> i64 {
    let mut res = origin * 2048;
    res = mix(origin, res);
    prune(res)
}

#[inline]
fn mix(n1: i64, n2: i64) -> i64 {
    n1 ^ n2
}

#[inline]
fn prune(n: i64) -> i64 {
    n % 16777216
}

#[inline]
fn process_step(origin: i64) -> i64 {
    let s1 = first_proc(origin);
    let s2 = second_proc(s1);
    third_prco(s2)
}

fn proc_num(mut origin: i64) -> i64 {
    for _ in 0..2000 {
        origin = process_step(origin);
    }
    origin
}

fn fill_map(mut origin: i64, seq_score_map: &mut HashMap<Vec<i8>, i64>) {
    let mut prices = Vec::with_capacity(2001);
    let mut diffs = Vec::with_capacity(2000);

    prices.push(origin % 10);

    let mut checked: HashSet<Vec<i8>> = HashSet::new();

    for idx in 0..2000 {
        origin = process_step(origin);
        let price = origin % 10;
        let diff = price - prices.last().unwrap();
        prices.push(price);
        diffs.push(diff as i8);

        if idx < 3 {
            continue;
        }

        let seq = diffs[diffs.len() - 4..].to_vec();

        // assert_eq!(seq.len(), 4);
        // assert!(seq.iter().all(|&n| n != INVLID));

        if checked.contains(&seq) {
            continue;
        }

        *seq_score_map.entry(seq.clone()).or_insert(0) += price;
        checked.insert(seq);
    }
}

fn calc_part2(input: &str) -> i64 {
    let buyers = parse(input);
    let mut final_seq_num_map: HashMap<Vec<i8>, i64> = HashMap::new();

    buyers
        .into_iter()
        .for_each(|b| fill_map(b, &mut final_seq_num_map));

    *final_seq_num_map.values().max().unwrap()
}

fn calc_nums(input: &str) -> i64 {
    let nums = parse(input);
    nums.into_par_iter().map(proc_num).sum()
}

fn part_1(input: &'static str) {
    let res = calc_nums(input);
    println!("Part 1 answer is {res}");
}

fn part_2(input: &'static str) {
    let ans = calc_part2(input);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "22").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
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

