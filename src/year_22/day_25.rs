use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Snafu {
    MinusTwo = -2,
    MinusOne = -1,
    Zero = 0,
    One = 1,
    Two = 2,
}

impl From<char> for Snafu {
    fn from(value: char) -> Self {
        match value {
            '=' => Snafu::MinusTwo,
            '-' => Snafu::MinusOne,
            '0' => Snafu::Zero,
            '1' => Snafu::One,
            '2' => Snafu::Two,
            _ => unreachable!(),
        }
    }
}

fn snafu_to_decimal(value: &str) -> isize {
    let nums: Vec<Snafu> = value.chars().map(Snafu::from).collect();

    nums.iter()
        .enumerate()
        .map(|(i, snafu)| 5_isize.pow((nums.len() - i - 1) as u32) * (*snafu as isize))
        .sum()
}

fn sum_decimal(input: &str) -> isize {
    input.lines().map(snafu_to_decimal).sum()
}

fn decimal_to_snafu(mut num: isize) -> String {
    let mut chars = Vec::new();
    while num > 0 {
        let reminder = num % 5;
        num /= 5;
        let char = match reminder {
            2 => '2',
            1 => '1',
            0 => '0',
            3 => {
                num += 1;
                '='
            }
            4 => {
                num += 1;
                '-'
            }
            _ => unreachable!(),
        };

        chars.insert(0, char);
    }

    chars.iter().collect()
}

fn part_1() {
    let input = read_text_from_file("22", "25");
    let decimal = sum_decimal(&input);
    let snafu = decimal_to_snafu(decimal);

    println!("Part 1 answer is {snafu}");
}
fn part_2() {
    println!("Congratulations!!!!");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(sum_decimal(INPUT), 4890);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(4890).as_str(), "2=-1=0");
    }
}
