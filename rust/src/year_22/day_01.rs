use crate::utls::read_text_from_file;
use itertools::Itertools;

fn part_1() {
    let input = read_text_from_file("22", "01");
    let max_cal = input
        .split("\n\n")
        .map(|arr| {
            arr.split('\n')
                .map(|num| num.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("Most calories are {max_cal}");
}

fn part_2() {
    let input = read_text_from_file("22", "01");
    let max_cal: i32 = input
        .split("\n\n")
        .map(|arr| {
            arr.split('\n')
                .map(|num| num.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Calories from top three are {max_cal}");
}

pub fn run() {
    part_1();
    part_2();
}
