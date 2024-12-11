use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

#[inline]
fn is_digit_count_even(num: u64) -> Option<(u64, u64)> {
    if num == 0 {
        return None;
    }
    let count = num.ilog10() + 1;
    if count % 2 == 0 {
        let half_digits = count / 2;
        let divisor = 10u64.pow(half_digits);
        let left = num / divisor;
        let right = num % divisor;
        return Some((left, right));
    }

    None
}

fn blink_after_count(input: &str, max_count: usize) -> u64 {
    let nums = parse(input);

    let mut num_counts = HashMap::new();

    for num in nums {
        *num_counts.entry(num).or_insert(0_u64) += 1;
    }

    for _i in 0..max_count {
        let mut new_map = HashMap::new();
        for (&num, &count) in num_counts.iter() {
            if num == 0 {
                *new_map.entry(1).or_insert(0) += count;
            } else if let Some((n1, n2)) = is_digit_count_even(num) {
                *new_map.entry(n1).or_insert(0) += count;
                *new_map.entry(n2).or_insert(0) += count;
            } else {
                *new_map.entry(num * 2024).or_insert(0) += count;
            }
        }

        num_counts = new_map
    }

    num_counts.values().sum()
}

fn part_1(input: &'static str) {
    let ans = blink_after_count(input, 25);
    println!("Part 1 answer is {ans}")
}

fn part_2(input: &'static str) {
    let ans = blink_after_count(input, 75);
    println!("Part 1 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "11").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_solution() {
        assert!(is_digit_count_even(0).is_none());
        assert!(is_digit_count_even(1).is_none());
        assert!(is_digit_count_even(100).is_none());
        assert_eq!(is_digit_count_even(10), Some((1, 0)));
        assert_eq!(is_digit_count_even(1000), Some((10, 0)));
        assert_eq!(is_digit_count_even(123456), Some((123, 456)));

        let blinks = blink_after_count(INPUT, 25);
        assert_eq!(blinks, 55312)
    }
}
