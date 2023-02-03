use std::collections::HashSet;

use itertools::Itertools;

use crate::utls::read_text_from_file;

fn get_char_priority(ch: char) -> usize {
    match ch {
        'a'..='z' => ch as usize - 'a' as usize + 1,
        'A'..='Z' => ch as usize - 'A' as usize + 27,
        _ => unreachable!("invalid input"),
    }
}

fn calc_err_priorities(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(first, second)| first.chars().filter(|ch| second.contains(*ch)).unique())
        .map(get_char_priority)
        .sum()
}

fn part_1() {
    let input = read_text_from_file("22", "03");
    let sum = calc_err_priorities(&input);

    println!("Part one answer is {sum}");
}

fn calc_groups_badges(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|mut chunk| {
            let set_1: HashSet<char> = chunk.next().unwrap().chars().collect();
            let set_2: HashSet<char> = chunk.next().unwrap().chars().collect();
            let set_3: HashSet<char> = chunk.next().unwrap().chars().collect();

            let set_1_2: HashSet<char> = set_1.intersection(&set_2).cloned().collect();

            set_1_2
                .intersection(&set_3)
                .cloned()
                .collect::<HashSet<char>>()
        })
        .map(get_char_priority)
        .sum()
}

fn part_2() {
    let input = read_text_from_file("22", "03");
    let sum = calc_groups_badges(&input);

    println!("Part two answer is {sum}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_calc_err() {
        assert_eq!(calc_err_priorities(INPUT), 157);
    }

    #[test]
    fn test_calc_groups() {
        assert_eq!(calc_groups_badges(INPUT), 70);
    }
}
