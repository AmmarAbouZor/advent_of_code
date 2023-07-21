use std::collections::HashMap;

use crate::utls::read_text_from_file;

fn calc_least_fuel(input: &str) -> usize {
    let poses: Vec<usize> = input
        .trim_end_matches('\n')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let min = poses.iter().min().unwrap();
    let max = poses.iter().max().unwrap();

    let mut min_fuel = usize::MAX;

    for num in *min..=*max {
        let fuel = poses.iter().map(|n| n.abs_diff(num)).sum();

        min_fuel = min_fuel.min(fuel)
    }

    min_fuel
}

fn calc_diff_cost(num: usize) -> usize {
    (1..=num).sum()
}

fn calc_least_fuel_2(input: &str) -> usize {
    let poses: Vec<usize> = input
        .trim_end_matches('\n')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let min = poses.iter().min().unwrap();
    let max = poses.iter().max().unwrap();

    let mut min_fuel = usize::MAX;

    let mut diff_map = HashMap::new();

    for num in *min..=*max {
        let fuel = poses
            .iter()
            .map(|n| {
                let diff = n.abs_diff(num);
                if let Some(cost) = diff_map.get(&diff) {
                    return *cost;
                }
                let cost = calc_diff_cost(diff);
                diff_map.insert(diff, cost);
                cost
            })
            .sum();

        min_fuel = min_fuel.min(fuel)
    }

    min_fuel
}

fn part_1() {
    let input = read_text_from_file("21", "07");
    let answer = calc_least_fuel(&input);

    println!("Part 1 answer is {answer}")
}

fn part_2() {
    let input = read_text_from_file("21", "07");
    let answer = calc_least_fuel_2(&input);

    println!("Part 2 answer is {answer}")
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_least_fuel(INPUT), 37)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_least_fuel_2(INPUT), 168)
    }
}

