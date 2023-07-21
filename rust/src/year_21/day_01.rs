use crate::utls::read_text_from_file;

fn calc_increases(input: &str) -> usize {
    let mut prev = u16::MAX;
    let mut count = 0;
    input
        .lines()
        .flat_map(|line| line.parse::<u16>())
        .for_each(|num| {
            if num > prev {
                count += 1;
            }
            prev = num
        });

    count
}

fn calc_increases_windows(input: &str) -> usize {
    let values: Vec<u16> = input.lines().flat_map(|line| line.parse()).collect();

    let mut prev = u16::MAX;
    let mut count = 0;

    values
        .windows(3)
        .map(|win| win.iter().sum())
        .for_each(|num| {
            if num > prev {
                count += 1;
            }
            prev = num
        });

    count
}

fn part_1() {
    let input = read_text_from_file("21", "01");
    let answer = calc_increases(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "01");
    let answer = calc_increases_windows(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_increases(INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_increases_windows(INPUT), 5);
    }
}
