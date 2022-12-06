#![allow(warnings, unused)]

use std::{
    fs,
    io::{self, BufRead},
};

pub fn read_lines_from_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).unwrap();

    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}

pub fn get_input_path(year: &str, day: &str) -> String {
    format!("input/year_{year}/day_{day}.txt")
}

pub fn read_text_from_file(year: &str, day: &str) -> String {
    let path = get_input_path(year, day);

    fs::read_to_string(path).unwrap()
}
