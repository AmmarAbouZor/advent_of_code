use std::collections::HashMap;

use crate::utls::read_text_from_file;
type Words = Vec<Vec<char>>;
fn get_input() -> Words {
    read_text_from_file("16", "06")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn get_most_frequent_char(words: &Words, index: usize) -> char {
    let mut chars_map = HashMap::new();
    for word in words {
        let count = chars_map.entry(word[index]).or_insert(0);
        *count += 1;
    }

    chars_map
        .iter()
        .max_by_key(|pair| pair.1)
        .unwrap()
        .0
        .to_owned()
}

fn get_least_frequent_char(words: &Words, index: usize) -> char {
    let mut chars_map = HashMap::new();
    for word in words {
        let count = chars_map.entry(word[index]).or_insert(0);
        *count += 1;
    }

    chars_map
        .iter()
        .min_by_key(|pair| pair.1)
        .unwrap()
        .0
        .to_owned()
}

fn part_1() {
    let words = get_input();

    let message: String = (0..words.first().unwrap().len())
        .map(|index| get_most_frequent_char(&words, index))
        .collect();

    println!("Message is {message}");
}

fn part_2() {
    let words = get_input();

    let message: String = (0..words.first().unwrap().len())
        .map(|index| get_least_frequent_char(&words, index))
        .collect();

    print!("Message part_2 is {message}");
}
pub fn run() {
    part_1();
    part_2();
}
