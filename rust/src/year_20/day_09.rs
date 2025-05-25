use std::collections::{HashSet, VecDeque};

use itertools::{Itertools, MinMaxResult::MinMax};

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

fn find_contiguous_set(input: &str, target: isize) -> isize {
    let nums: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();

    for idx in 0..nums.len() {
        let mut sum = nums[idx];
        let mut next_idx = idx + 1;

        while sum < target {
            sum += nums[next_idx];
            if sum == target {
                if let &MinMax(min, max) = &nums[idx..=next_idx].iter().minmax() {
                    return min + max;
                }
            }
            next_idx += 1;
        }
    }

    unreachable!()
}

fn part_1(input: &str) -> isize {
    let answer = find_first_invalid(input, 26);
    println!("Part 1 answer is {answer}");

    answer
}

fn part_2(input: &str, answer_1: isize) {
    let answer = find_contiguous_set(input, answer_1);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("20", "09");
    let answer_1 = part_1(&input);
    part_2(&input, answer_1);
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
        assert_eq!(find_contiguous_set(INPUT, 127), 62);
    }
}
