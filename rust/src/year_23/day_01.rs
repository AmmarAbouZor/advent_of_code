use crate::utls::read_text_from_file;

fn find_num(line: &str) -> u32 {
    let first_num = line.chars().find(|ch| ch.is_numeric()).unwrap();
    let last_num = line.chars().rev().find(|ch| ch.is_numeric()).unwrap();

    first_num.to_digit(10).unwrap() * 10 + last_num.to_digit(10).unwrap()
}

fn calc_sum(input: &str) -> u32 {
    input.lines().map(find_num).sum()
}

fn part_1(input: &str) {
    let answer = calc_sum(input);

    println!("Part 1 Answer is {answer}");
}
fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "01");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_solution() {
        assert_eq!(calc_sum(INPUT), 142);
    }
}

