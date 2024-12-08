use itertools::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Sum,
    Mul,
    Combine,
}

impl Op {
    fn all_part1() -> &'static [Op] {
        &[Op::Sum, Op::Mul]
    }

    fn all_part2() -> &'static [Op] {
        &[Op::Sum, Op::Mul, Op::Combine]
    }

    fn apply(self, n1: i64, n2: i64) -> i64 {
        match self {
            Op::Sum => n1 + n2,
            Op::Mul => n1 * n2,
            Op::Combine => combine_numbers(n1, n2),
            // Op::Combine => format!("{}{}", n1, n2).parse().unwrap(),
        }
    }
}

// Better performance than concatenating then parsing string
fn combine_numbers(a: i64, b: i64) -> i64 {
    let mut b_digits = 0;
    let mut temp = b;
    while temp > 0 {
        temp /= 10;
        b_digits += 1;
    }
    a * 10_i64.pow(b_digits) + b
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(": ").unwrap();
            let sum = p1.parse().unwrap();
            let parts = p2.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (sum, parts)
        })
        .collect()
}

fn is_valid(target: i64, parts: &[i64], all_ops: &[Op]) -> bool {
    let len = parts.len() - 1;

    repeat_n(all_ops.iter(), len)
        .multi_cartesian_product()
        .any(|ops| {
            let mut ops = ops.into_iter();
            let sum = parts
                .iter()
                .copied()
                .reduce(|acc, e| {
                    let op = ops.next().unwrap();
                    op.apply(acc, e)
                })
                .unwrap();
            sum == target
        })
}

fn valid_sum_part1(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .into_par_iter()
        .filter(|eq| is_valid(eq.0, eq.1.as_slice(), Op::all_part1()))
        .map(|eq| eq.0)
        .sum()
}

fn part_1(input: &'static str) {
    let sum = valid_sum_part1(input);
    println!("Part 1 answer is {sum}");
}

fn valid_sum_part2(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .into_par_iter()
        .filter(|eq| is_valid(eq.0, eq.1.as_slice(), Op::all_part2()))
        .map(|eq| eq.0)
        .sum()
}

fn part_2(input: &'static str) {
    let sum2 = valid_sum_part2(input);

    println!("Part 2 answer is {sum2}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "07").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_solution() {
        let sum1 = valid_sum_part1(INPUT);
        assert_eq!(sum1, 3749);

        let sum2 = valid_sum_part2(INPUT);
        assert_eq!(sum2, 11387);
    }
}
