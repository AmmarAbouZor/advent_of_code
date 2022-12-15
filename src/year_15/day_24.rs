#![allow(warnings, unused)]

use rand::{rngs::ThreadRng, seq::SliceRandom};

const INPUT: [usize; 29] = [
    1, 2, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
    103, 107, 109, 113,
];

struct Solution {
    count: usize,
    qe: usize,
}

impl Solution {
    fn new(count: usize, qe: usize) -> Self {
        Self { count, qe }
    }
}

fn get_random_solution_3(
    mut packages: Vec<usize>,
    target_weight: usize,
    rng: &mut ThreadRng,
) -> Option<Solution> {
    packages.shuffle(rng);

    let mut group_1 = vec![];
    let mut gr_1_sum = 0;
    let mut gr_2_sum = 0;
    let mut gr_3_sum = 0;

    while let Some(num) = packages.pop() {
        match num {
            weight if weight + gr_1_sum <= target_weight => {
                gr_1_sum += weight;
                group_1.push(weight);
            }
            weight if weight + gr_2_sum <= target_weight => gr_2_sum += weight,
            weight if weight + gr_3_sum <= target_weight => gr_3_sum += weight,
            _ => return None,
        }
    }

    Some(Solution::new(group_1.len(), group_1.iter().product()))
}

fn get_random_solution_4(
    mut packages: Vec<usize>,
    target_weight: usize,
    rng: &mut ThreadRng,
) -> Option<Solution> {
    packages.shuffle(rng);

    let mut group_1 = vec![];
    let mut gr_1_sum = 0;
    let mut gr_2_sum = 0;
    let mut gr_3_sum = 0;
    let mut gr_4_sum = 0;

    while let Some(num) = packages.pop() {
        match num {
            weight if weight + gr_1_sum <= target_weight => {
                gr_1_sum += weight;
                group_1.push(weight);
            }
            weight if weight + gr_2_sum <= target_weight => gr_2_sum += weight,
            weight if weight + gr_3_sum <= target_weight => gr_3_sum += weight,
            weight if weight + gr_4_sum <= target_weight => gr_4_sum += weight,
            _ => return None,
        }
    }

    Some(Solution::new(group_1.len(), group_1.iter().product()))
}

fn get_qe_ideal_conf(packages: Vec<usize>, gr_count: usize) -> usize {
    let mut min_count = usize::MAX;
    let mut min_qe = usize::MAX;

    let mut rng = rand::thread_rng();
    let target = packages.iter().sum::<usize>() / gr_count;
    let func = if gr_count == 3 {
        get_random_solution_3
    } else {
        get_random_solution_4
    };
    for _ in 0..1000_000 {
        if let Some(solution) = func(packages.clone(), target, &mut rng) {
            if solution.count <= min_count {
                min_count = solution.count;
                min_qe = min_qe.min(solution.qe);
            }
        }
    }

    min_qe
}

fn part_1() {
    let qe = get_qe_ideal_conf(INPUT.to_vec(), 3);

    println!("part 1: qe of ideal config is {qe}");
}
fn part_2() {
    let qe = get_qe_ideal_conf(INPUT.to_vec(), 4);

    println!("part 2: qe of ideal config is {qe}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_packages() -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
    }
}
