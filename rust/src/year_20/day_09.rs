use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use crate::utls::read_text_from_file;

fn find_first_invalid(input: &str, len: usize) -> isize {
    let mut binding = input.lines();
    let lines_iter = binding.by_ref();

    let mut current_nums: VecDeque<isize> = lines_iter
        .take(len)
        .map(|num| num.parse().unwrap())
        .collect();

    loop {
        let new_num: isize = lines_iter.next().and_then(|num| num.parse().ok()).unwrap();

        if check_two_sum(new_num, &current_nums) {
            let _ = current_nums.pop_front().unwrap();
            current_nums.push_back(new_num);
        } else {
            return new_num;
        }
    }
}

fn check_two_sum(target: isize, nums: &VecDeque<isize>) -> bool {
    let mut diff_set = HashSet::new();

    for num in nums {
        if diff_set.contains(num) {
            return true;
        }

        diff_set.insert(target - *num);
    }

    false
}

fn part_1() {
    let input = read_text_from_file("20", "09");
    let answer = find_first_invalid(&input, 25);

    println!("Part 1 answer is {answer}")
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_() {
        assert_eq!(find_first_invalid(INPUT, 5), 127);
    }
}

