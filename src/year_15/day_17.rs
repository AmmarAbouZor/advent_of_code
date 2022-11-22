#![allow(warnings, unused)]

use itertools::Itertools;

const INPUT: [u16; 20] = [
    33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
];

fn get_range_min(goal: u16, mut input: Vec<u16>) -> usize {
    input.sort();
    let mut sum = 0;
    for (i, num) in input.iter().rev().enumerate() {
        sum += num;
        if sum >= goal {
            return i + 1;
        }
    }

    panic!("must not get to here")
}
fn get_range_max(goal: u16, mut input: Vec<u16>) -> usize {
    input.sort();

    while input.iter().sum::<u16>() > goal {
        input.pop();
    }

    input.len()
}

fn calc_combi_count(goal: u16, input: Vec<u16>) -> usize {
    let min = get_range_min(goal, input.clone());
    let max = get_range_max(goal, input.clone());

    let mut combi_count = 0;
    for i in min..=max {
        combi_count += input
            .iter()
            .combinations(i)
            .filter(|vec| vec.clone().into_iter().sum::<u16>() == goal)
            .count();
    }

    combi_count
}

fn calc_min_combi_count(goal: u16, input: Vec<u16>) -> usize {
    let min = get_range_min(goal, input.clone());

    input
        .iter()
        .combinations(min)
        .filter(|vec| vec.clone().into_iter().sum::<u16>() == goal)
        .count()
}

fn part_1() {
    let mut input = INPUT.to_vec();
    let combinations = calc_combi_count(150, input);

    println!("combinations are {combinations}");
}

fn part_2() {
    let mut input = INPUT.to_vec();
    let combinations = calc_min_combi_count(150, input);

    println!("min combinations are {combinations}");
}
pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_input() -> Vec<u16> {
        vec![20, 15, 10, 5, 5]
    }

    #[test]
    fn test_ranges() {
        let mut input = get_input();
        assert_eq!(get_range_min(25, input), 2);
        let mut input = get_input();
        assert_eq!(get_range_max(25, input), 3);
    }
    #[test]
    fn test_combi_count() {
        let input = get_input();
        assert_eq!(calc_combi_count(25, input), 4);
    }
}
