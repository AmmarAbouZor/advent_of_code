use std::{num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Marker {
    offset: usize,
    repeat: usize,
}

impl FromStr for Marker {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matches: &[_] = &['(', ')'];
        let (offset, repeat) = input.trim_matches(matches).split_once('x').unwrap();
        let offset = offset.parse()?;
        let repeat = repeat.parse()?;

        Ok(Marker { offset, repeat })
    }
}

fn read_input() -> String {
    read_text_from_file("16", "09")
}

fn decompress(input: &str) -> String {
    let mut output = vec![];
    let input: Vec<char> = input.chars().collect();

    let mut i = 0;

    while i < input.len() {
        if input[i] == '(' {
            let end_index = input[i..].iter().position(|ch| *ch == ')').unwrap() + i;
            let marker: String = input[i..=end_index].iter().collect();
            let marker: Marker = marker.parse().unwrap();

            let chunk_start = end_index + 1;
            let chunk = &input[chunk_start..marker.offset + chunk_start];
            for _ in 0..marker.repeat {
                output.extend_from_slice(chunk);
            }
            i = chunk_start + marker.offset;
        } else {
            output.push(input[i]);
            i += 1;
        }
    }

    let output = output.iter().collect::<String>();

    output
}

fn part_1() {
    let input = read_input();
    let len = decompress(&input).len();

    println!("len is {len}");
}

fn decompress_recr(input: &[char]) -> usize {
    let mut output = 0;

    let mut i = 0;

    while i < input.len() {
        if input[i] == '(' {
            let end_index = input[i..].iter().position(|ch| *ch == ')').unwrap() + i;
            let marker: String = input[i..=end_index].iter().collect();
            let marker: Marker = marker.parse().unwrap();

            let chunk_start = end_index + 1;
            let chunk = &input[chunk_start..marker.offset + chunk_start];

            let add = if chunk.contains(&'(') {
                decompress_recr(chunk)
            } else {
                chunk.len()
            };
            for _ in 0..marker.repeat {
                output += add;
            }
            i = chunk_start + marker.offset;
        } else {
            output += 1;
            i += 1;
        }
    }

    output
}

fn part_2() {
    let input: Vec<char> = read_input().chars().collect();

    let count = decompress_recr(&input);

    println!("count is {count}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_floor() {
        assert_eq!(decompress("ADVENT").len(), 6);
        assert_eq!(decompress("A(1x5)BC").len(), 7);
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG").len(), 11);
        assert_eq!(decompress("(3x3)XYZ").len(), 9);
        assert_eq!(decompress("(6x1)(1x3)A").len(), 6);
        assert_eq!(decompress("X(8x2)(3x3)ABCY").len(), 18);
    }
}
