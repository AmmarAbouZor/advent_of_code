use crate::utls::read_text_from_file;

fn find_num_digits(line: &str) -> u32 {
    let first_num = line.chars().find(|ch| ch.is_numeric()).unwrap();
    let last_num = line.chars().rev().find(|ch| ch.is_numeric()).unwrap();

    first_num.to_digit(10).unwrap() * 10 + last_num.to_digit(10).unwrap()
}

fn calc_sum_digits(input: &str) -> u32 {
    input.lines().map(find_num_digits).sum()
}

fn find_num_words(line: &str) -> u32 {
    let mut first_num = None;
    for idx in 0..line.len() {
        first_num = line[idx..idx + 1].parse().ok();
        if first_num.is_some() {
            break;
        }
        first_num = word_to_num(&line[idx..]);
        if first_num.is_some() {
            break;
        }
    }
    let mut last_num = None;
    for rev_idx in (0..line.len()).rev() {
        last_num = line[rev_idx..rev_idx + 1].parse().ok();
        if last_num.is_some() {
            break;
        }
        last_num = word_to_num(&line[rev_idx..]);
        if last_num.is_some() {
            break;
        }
    }

    first_num.unwrap() * 10 + last_num.unwrap()
}

fn word_to_num(word: &str) -> Option<u32> {
    if word.starts_with("one") {
        Some(1)
    } else if word.starts_with("two") {
        Some(2)
    } else if word.starts_with("three") {
        Some(3)
    } else if word.starts_with("four") {
        Some(4)
    } else if word.starts_with("five") {
        Some(5)
    } else if word.starts_with("six") {
        Some(6)
    } else if word.starts_with("seven") {
        Some(7)
    } else if word.starts_with("eight") {
        Some(8)
    } else if word.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn calc_sum_words(input: &str) -> u32 {
    input.lines().map(find_num_words).sum()
}

fn part_1(input: &str) {
    let answer = calc_sum_digits(input);

    println!("Part 1 Answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_sum_words(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "01");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_solution() {
        assert_eq!(calc_sum_digits(INPUT_1), 142);
        assert_eq!(calc_sum_words(INPUT_2), 281);
    }
}

