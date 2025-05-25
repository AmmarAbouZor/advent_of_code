use crate::utls::read_text_from_file;

fn fill_levels(nums: Vec<isize>) -> Vec<Vec<isize>> {
    let mut levels = vec![nums];
    let mut last_level = levels.last().unwrap();
    while last_level.iter().any(|num| *num != 0) {
        let new_level: Vec<isize> = last_level
            .windows(2)
            .map(|nums| nums[1] - nums[0])
            .collect();
        levels.push(new_level);
        last_level = levels.last().unwrap();
    }

    levels
}

fn calc_extrapolated(nums: Vec<isize>) -> isize {
    let levels = fill_levels(nums);

    levels
        .iter()
        .rev()
        .skip(1)
        .fold(0, |extrap, level| extrap + *level.last().unwrap())
}

fn calc_extra_revers(nums: Vec<isize>) -> isize {
    let levels = fill_levels(nums);

    levels
        .iter()
        .rev()
        .skip(1)
        .fold(0, |extrap, level| *level.first().unwrap() - extrap)
}

fn calc_sum<F>(input: &str, calc_func: F) -> isize
where
    F: FnMut(Vec<isize>) -> isize,
{
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(calc_func)
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_sum(input, calc_extrapolated);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_sum(input, calc_extra_revers);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "09");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_solution() {
        assert_eq!(calc_sum(INPUT, calc_extrapolated), 114);
        assert_eq!(calc_sum(INPUT, calc_extra_revers), 2);
    }
}
