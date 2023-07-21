#![allow(warnings, unused)]

use std::char;

use crate::utls::read_lines_from_file;

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut chars_iter = input.chars();
    let mut last_char = chars_iter.next().unwrap();
    let mut repeat_count = 1;
    for char in chars_iter {
        if last_char == char {
            repeat_count += 1;
        } else {
            let repeat_char = char::from_digit(repeat_count, 10).unwrap();
            output.push(repeat_char);
            output.push(last_char);
            last_char = char;
            repeat_count = 1;
        }
    }

    let repeat_char = char::from_digit(repeat_count, 10).unwrap();
    output.push(repeat_char);
    output.push(last_char);

    output
}

pub fn run() {
    let mut text = String::from("1113222113");

    for _ in 0..40 {
        text = look_and_say(&text);
    }

    println!("length after looping 40 times is {}", text.len());
    for _ in 0..10 {
        text = look_and_say(&text);
    }

    println!("length after 50 times is {}", text.len());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
