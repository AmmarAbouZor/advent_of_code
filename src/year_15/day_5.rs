#![allow(warnings, unused)]
use std::{
    fs,
    io::{self, BufRead},
};

use crate::utls::read_lines_from_file;

const FORBIDDEN_STRINGS: &'static [&str] = &["ab", "cd", "pq", "xy"];
const VOWELS: &'static [char] = &['a', 'e', 'i', 'o', 'u'];

fn have_forbidden_words(word: &str) -> bool {
    FORBIDDEN_STRINGS.iter().any(|forbid| word.contains(forbid))
}

fn contains_three_vowels(word: &str) -> bool {
    let mut count = 0usize;

    for ch in word.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
        if count == 3 {
            return true;
        }
    }

    false
}

fn has_two_chars_in_row(word: &str) -> bool {
    let mut last_char = ' ';
    for ch in word.chars() {
        if last_char == ch {
            return true;
        } else {
            last_char = ch;
        }
    }

    false
}

fn is_string_nice(word: &str) -> bool {
    !have_forbidden_words(&word) && contains_three_vowels(&word) && has_two_chars_in_row(&word)
}

fn contain_pair_twice(word: &str) -> bool {
    if word.len() < 2 {
        panic!("word too short");
    }
    let mut current_index = 0usize;

    while current_index + 2 < word.len() {
        let slice_to_match = &word[current_index..=current_index + 1];
        let mut matching_index = current_index + 2;
        while matching_index + 1 < word.len() {
            let next_slice = &word[matching_index..=matching_index + 1];
            if slice_to_match == next_slice {
                return true;
            }

            matching_index += 1;
        }

        current_index += 1;
    }

    false
}

fn has_letter_repeat_between(word: &str) -> bool {
    let mut current_index = 0usize;

    while current_index + 2 < word.len() {
        let current_char = &word[current_index..=current_index];

        let next_index = current_index + 2;
        let char_to_match = &word[next_index..=next_index];
        if current_char == char_to_match {
            return true;
        }

        current_index += 1;
    }

    false
}

fn is_word_good_v2(word: &str) -> bool {
    contain_pair_twice(&word) && has_letter_repeat_between(&word)
}

pub fn run() {
    let nice_words = read_lines_from_file(r"src/year_15/day_5.txt")
        .into_iter()
        .filter(|word| is_string_nice(word))
        .count();

    println!("number of nice words is: {nice_words}");

    let file = fs::File::open(r"src/year_15/day_5.txt").unwrap();

    let nice_words_2 = read_lines_from_file(r"src/year_15/day_5.txt")
        .into_iter()
        .filter(|word| is_word_good_v2(word))
        .count();

    println!("number of nice words v2 is: {nice_words_2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nice_string() {
        assert!(is_string_nice("ugknbfddgicrmopn"));
        assert!(is_string_nice("aaa"));
        assert!(!is_string_nice("jchzalrnumimnmhp"));
        assert!(!is_string_nice("haegwjzuvuyypxyu"));
        assert!(!is_string_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_nice_string_v2() {
        assert!(is_word_good_v2("qjhvhtzxzqqjkmpb"));
        assert!(is_word_good_v2("xxyxx"));
        assert!(!is_word_good_v2("uurcxstgmygtbstg"));
        assert!(!is_word_good_v2("ieodomkazucvgmuy"));
    }
}
