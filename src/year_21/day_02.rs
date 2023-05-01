use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Cmd {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl From<&str> for Cmd {
    fn from(value: &str) -> Self {
        let (name, num) = value.split_once(' ').unwrap();
        let num = num.parse().unwrap();

        match name {
            "forward" => Cmd::Forward(num),
            "down" => Cmd::Down(num),
            "up" => Cmd::Up(num),
            _ => unreachable!(),
        }
    }
}

fn depth_hight_mul(input: &str) -> isize {
    let (pos, depth) = input
        .lines()
        .map(Cmd::from)
        .fold((0, 0), |(pos, depth), cmd| match cmd {
            Cmd::Forward(num) => (pos + num, depth),
            Cmd::Down(num) => (pos, depth + num),
            Cmd::Up(num) => (pos, depth - num),
        });

    pos * depth
}

fn depth_hight_aim(input: &str) -> isize {
    let mut aim = 0;
    let (pos, depth) = input
        .lines()
        .map(Cmd::from)
        .fold((0, 0), |(pos, depth), cmd| match cmd {
            Cmd::Forward(num) => (pos + num, depth + (num * aim)),
            Cmd::Down(num) => {
                aim += num;
                (pos, depth)
            }
            Cmd::Up(num) => {
                aim -= num;
                (pos, depth)
            }
        });

    pos * depth
}

fn part_1() {
    let input = read_text_from_file("21", "02");
    let answer = depth_hight_mul(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "02");
    let answer = depth_hight_aim(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_1() {
        assert_eq!(depth_hight_mul(INPUT), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(depth_hight_aim(INPUT), 900);
    }
}

