use regex::Regex;

fn calc_mul(input: &str) -> usize {
    const PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(PATTERN).unwrap();

    regex
        .captures_iter(input)
        .map(|mul| mul.extract::<2>())
        .map(|(_, [n1, n2])| {
            let n1: usize = n1.parse().unwrap();
            let n2: usize = n2.parse().unwrap();
            n1 * n2
        })
        .sum()
}

fn filter_calc(input: &str) -> usize {
    let mut chunks = input.split("don't()");
    let mut sum = calc_mul(chunks.next().unwrap());

    for chunk in chunks {
        if let Some((_ignore, take)) = chunk.split_once("do()") {
            sum += calc_mul(take);
        }
    }

    sum
}

fn part_1(input: &'static str) {
    let mul = calc_mul(input);
    println!("Part 1 answer is {mul}");
}

fn part_2(input: &'static str) {
    let fil = filter_calc(input);
    println!("Part 2 answer is {fil}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "03").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_solution() {
        let mul = calc_mul(INPUT);
        assert_eq!(mul, 161);

        let fil = filter_calc(INPUT_2);
        assert_eq!(fil, 48)
    }
}

