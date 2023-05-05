use crate::utls::read_text_from_file;

fn calc_power(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let (gamma, epsilon) = get_most_least(&lines);

    let gamma_val = parse_binary_string(&gamma);
    let epsilon_val = parse_binary_string(&epsilon);

    gamma_val * epsilon_val
}

fn get_most_least(lines: &Vec<&str>) -> (String, String) {
    let mut ones_sum = vec![0; lines[0].len()];

    lines.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(index, ch)| {
            if ch == '1' {
                ones_sum[index] += 1;
            }
        })
    });

    let mut most: String = String::new();
    let mut least: String = String::new();

    ones_sum.iter().for_each(|&num| {
        if num > lines.len() / 2 {
            most.push('1');
            least.push('0')
        } else {
            most.push('0');
            least.push('1')
        }
    });

    (most, least)
}

fn parse_binary_string(num_txt: &str) -> u32 {
    let mut val: u32 = 0;

    num_txt.chars().for_each(|ch| {
        val <<= 1;
        match ch {
            '1' => val |= 1,
            '0' => val |= 0,
            _ => unreachable!(),
        }
    });

    val
}

fn part_1() {
    let input = read_text_from_file("21", "03");
    let answer = calc_power(&input);

    println!("Part 1 answer is {answer}");
}

fn calc_life_support(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let most = filter_most(lines.clone());
    let least = filter_least(lines);

    let most_val = parse_binary_string(&most);
    let least_val = parse_binary_string(&least);

    most_val * least_val
}

fn filter_most(mut nums: Vec<&str>) -> String {
    let mut index = 0;
    while nums.len() > 1 {
        let ones_count = nums
            .iter()
            .filter(|num| num.chars().nth(index).unwrap() == '1')
            .count();

        let zeros_count = nums.len().checked_sub(ones_count).unwrap();

        if ones_count > zeros_count {
            nums.retain(|num| num.chars().nth(index).unwrap() == '1');
        } else if ones_count < zeros_count {
            nums.retain(|num| num.chars().nth(index).unwrap() == '0');
        } else if nums.len() == 2 {
            nums.retain(|num| num.chars().nth(index).unwrap() == '1');
        }

        index += 1;
    }

    nums[0].to_owned()
}

fn filter_least(mut nums: Vec<&str>) -> String {
    let mut index = 0;
    while nums.len() > 1 {
        let ones_count = nums
            .iter()
            .filter(|num| num.chars().nth(index).unwrap() == '1')
            .count();

        let zeros_count = nums.len().checked_sub(ones_count).unwrap();

        if ones_count < zeros_count {
            nums.retain(|num| num.chars().nth(index).unwrap() == '1');
        } else if ones_count > zeros_count {
            nums.retain(|num| num.chars().nth(index).unwrap() == '0');
        } else if nums.len() == 2 {
            nums.retain(|num| num.chars().nth(index).unwrap() == '0');
        }

        index += 1;
    }

    nums[0].to_owned()
}

fn part_2() {
    let input = read_text_from_file("21", "03");
    let answer = calc_life_support(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_power(INPUT), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_life_support(INPUT), 230);
    }
}

