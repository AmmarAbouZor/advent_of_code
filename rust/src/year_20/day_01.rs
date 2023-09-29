use std::collections::HashMap;

use crate::utls::read_text_from_file;

fn calc_sum_multi(input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let (num_1, num_2) = find_two_some(&nums, 2020).unwrap();

    num_1 * num_2
}

fn find_two_some(nums: &[usize], target: usize) -> Option<(usize, usize)> {
    let mut sums_map = HashMap::new();
    for num in nums {
        let to_target = target.wrapping_sub(*num);
        if let Some(&second_num) = sums_map.get(num) {
            return Some((*num, second_num));
        } else {
            sums_map.insert(to_target, *num);
        }
    }

    None
}

fn calc_three_some(input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    for (idx, num) in nums.iter().enumerate() {
        if let Some((num_1, num_2)) = find_two_some(&nums[idx + 1..], 2020 - *num) {
            return num * num_1 * num_2;
        }
    }

    unreachable!()
}

fn part_1() {
    let input = read_text_from_file("20", "01");
    let answer = calc_sum_multi(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "01");
    let answer = calc_three_some(input.as_str());

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1721
979
366
299
675
1456
";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_sum_multi(INPUT), 514579);
        assert_eq!(calc_three_some(INPUT), 241861950);
    }
}

