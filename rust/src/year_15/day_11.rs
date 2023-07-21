#![allow(warnings, unused)]

use std::ops::Range;
use std::str;

const START_LOWER: u8 = b'a';
const END_LOWER: u8 = b'z';
const LOWER_RANG: Range<u8> = START_LOWER..END_LOWER;
const FORBIDDEN_CHARS: [u8; 3] = [b'i', b'o', b'l'];

fn get_chars_array(word: &str) -> [u8; 8] {
    let mut chars_array = [0u8; 8];
    for (i, ch) in word.chars().enumerate() {
        chars_array[i] = ch as u8;
    }

    chars_array
}

fn get_word_from_array(array: &[u8; 8]) -> &str {
    str::from_utf8(array).unwrap()
}

fn set_next_array(array: &mut [u8; 8]) {
    let mut index = array.len() - 1;
    let mut in_range = false;
    while !in_range {
        array[index] = array[index] + 1;
        in_range = array[index] <= END_LOWER;
        if !in_range {
            array[index] = START_LOWER;
            index -= 1;
        }
    }
}

fn has_two_pairs(password: &[u8; 8]) -> bool {
    let mut pair_count = 0u8;
    let mut index = 0;
    while index < password.len() - 1 && pair_count < 2 {
        if password[index] == password[index + 1] {
            pair_count += 1;
            index += 1;
        }
        index += 1;
    }

    pair_count >= 2
}

fn has_increasing_triple(password: &[u8; 8]) -> bool {
    let mut has_increasing = false;
    let mut index = 0;
    while index < password.len() - 2 && !has_increasing {
        has_increasing = password[index + 1] == password[index] + 1
            && password[index + 2] == password[index + 1] + 1;
        index += 1;
    }

    has_increasing
}

fn has_invalid_chars(password: &[u8; 8]) -> bool {
    password.iter().any(|b| matches!(b, b'i' | b'o' | b'l'))
}

fn find_next_password(current: &str) -> String {
    let mut array = get_chars_array(&current);
    let mut valid = false;
    while !valid {
        set_next_array(&mut array);
        valid =
            !has_invalid_chars(&array) && has_two_pairs(&array) && has_increasing_triple(&array);
    }

    get_word_from_array(&array).into()
}

pub fn run() {
    let input = "hxbxwxba";
    let next_password = find_next_password(input);
    println!("next password is: {next_password}");
    let password_after = find_next_password(&next_password);
    println!("password after that is: {password_after}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_word() {
        check_next_word("aabbccdd", "aabbccde");
        check_next_word("aabbccdz", "aabbccea");
        check_next_word("aabbcczz", "aabbcdaa");
        check_next_word("aabbczzz", "aabbdaaa");
    }

    fn check_next_word(current: &str, next: &str) {
        let mut array = get_chars_array(current);
        set_next_array(&mut array);
        assert_eq!(get_word_from_array(&array), next);
    }

    #[test]
    fn test_has_two_pair() {
        assert!(has_two_pairs(&get_chars_array("aafdeewe")));
        assert!(!has_two_pairs(&get_chars_array("aaadhewe")));
        assert!(has_two_pairs(&get_chars_array("aaffeewe")));
    }

    #[test]
    fn test_three_increasing() {
        assert!(has_increasing_triple(&get_chars_array("abcdeewe")));
        assert!(has_increasing_triple(&get_chars_array("aqcdhxyz")));
        assert!(!has_increasing_triple(&get_chars_array("axcduewe")));
    }

    #[test]
    fn test_find_next_word() {
        assert_eq!(find_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(find_next_password("ghijklmn"), "ghjaabcc");
    }
}
