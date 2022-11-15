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
