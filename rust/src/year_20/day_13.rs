use std::{cmp::Reverse, collections::BinaryHeap};

use crate::utls::read_text_from_file;

fn get_min_diff(id: usize, target: usize) -> usize {
    let num_before = target / id;
    let first_arrive = id * (num_before + 1);

    first_arrive - target
}

fn get_earliest(input: &str) -> usize {
    let (target, ids) = input.split_once('\n').unwrap();
    let target: usize = target.parse().unwrap();
    let mut heap: BinaryHeap<_> = ids
        .split(',')
        .flat_map(|id| id.parse().ok())
        .map(|id| Reverse((get_min_diff(id, target), id)))
        .collect();

    heap.pop().map(|Reverse((diff, id))| diff * id).unwrap()
}

fn part_1(input: &str) {
    let answer_1 = get_earliest(input);

    println!("Part 1 answer is {answer_1}");
}
fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("20", "13");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part_1() {
        assert_eq!(get_earliest(INPUT), 295);
    }
}

