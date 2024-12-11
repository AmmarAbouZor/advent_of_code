fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
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

fn apply_blink(nums: Vec<u64>) -> Vec<u64> {
    let mut applied = Vec::with_capacity(nums.len() * 3 / 2);

    for num in nums {
        if num == 0 {
            applied.push(1);
        } else if let Some((n1, n2)) = is_digit_count_even(num) {
            applied.push(n1);
            applied.push(n2);
        } else {
            applied.push(num * 2024);
        }
    }

    applied
}

fn blink_after_count(input: &str, count: usize) -> usize {
    let mut nums = parse(input);
    for _ in 0..count {
        nums = apply_blink(nums);
    }

    nums.len()
}

fn part_1(input: &'static str) {
    let ans = blink_after_count(input, 25);
    println!("Part 1 answer is {ans}")
}

fn part_2(input: &'static str) {
    // let ans = blink_after_count(input, 75);
    // println!("Part 1 answer is {ans}")
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

