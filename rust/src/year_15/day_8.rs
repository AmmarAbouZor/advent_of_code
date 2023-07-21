#![allow(warnings, unused)]

use crate::utls::read_lines_from_file;

struct CharsCounts {
    total: usize,
    wanted: usize,
}

fn get_counts_from_line(line: String) -> CharsCounts {
    let chars: Vec<char> = line.chars().collect();
    let total = chars.len();
    let mut real = 0usize;
    let mut index = 1;
    while index < total - 1 {
        real += 1;
        let ch = chars[index];
        if ch == '\\' {
            index += 1;
            if chars[index] == 'x' {
                index += 2;
            }
        }
        index += 1;
    }

    CharsCounts {
        total,
        wanted: real,
    }
}

fn get_counts_from_line_extra(line: String) -> CharsCounts {
    let escapes_count = line.chars().filter(|ch| matches!(ch, '\\' | '\"')).count();

    let total = line.len();
    CharsCounts {
        total,
        wanted: total + escapes_count + 2,
    }
}

pub fn part_one() {
    let mut total = 0usize;
    let mut real = 0usize;
    read_lines_from_file(r"src/year_15/day_8.txt")
        .into_iter()
        .map(get_counts_from_line)
        .for_each(|counts| {
            total += counts.total;
            real += counts.wanted;
        });

    println!("part one difference is {}", total - real);
}

pub fn part_two() {
    let mut total = 0usize;
    let mut extra = 0usize;
    read_lines_from_file(r"src/year_15/day_8.txt")
        .into_iter()
        .map(get_counts_from_line_extra)
        .for_each(|counts| {
            total += counts.total;
            extra += counts.wanted;
        });

    println!("part two difference is {}", extra - total);
}

pub fn run() {
    part_one();
    part_two();
}
