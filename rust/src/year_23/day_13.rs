use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::utls::read_text_from_file;

#[derive(Debug, PartialEq, Eq)]
enum MirrorLine {
    Horizontal(usize),
    Vertical(usize),
}

impl MirrorLine {
    fn get_score(self) -> usize {
        match self {
            MirrorLine::Horizontal(val) => val * 100,
            MirrorLine::Vertical(val) => val,
        }
    }
}

fn find_mirror(chunk: &str) -> Vec<MirrorLine> {
    try_horizontal(chunk)
        .into_iter()
        .map(MirrorLine::Horizontal)
        .chain(try_vertical(chunk).into_iter().map(MirrorLine::Vertical))
        .collect()
}

fn find_smudged(chunk: &str) -> Vec<MirrorLine> {
    let original_lines = find_mirror(chunk);
    let original_line = original_lines.first().unwrap();
    let mut chunk = chunk.to_owned();

    for idx in 0..chunk.len() {
        if swap_string(&mut chunk, idx) {
            let mirrors = find_mirror(&chunk);
            if let Some(mirror_line) = mirrors.into_iter().find(|mir| mir != original_line) {
                return vec![mirror_line];
            }
            assert!(swap_string(&mut chunk, idx));
        }
    }

    unreachable!("Each smudged chunk must have mirror line. \n{chunk}")
}

fn swap_string(chunk: &mut str, idx: usize) -> bool {
    let ch_bytes = unsafe { chunk.as_bytes_mut() };

    match ch_bytes[idx] {
        b'.' => {
            ch_bytes[idx] = b'#';
            true
        }
        b'#' => {
            ch_bytes[idx] = b'.';
            true
        }
        b'\n' => false,
        invalid => unreachable!("Invalid input: '{}'", invalid as char),
    }
}

fn try_horizontal(chunk: &str) -> Vec<usize> {
    let mut mirrors = Vec::new();
    let lines: Vec<_> = chunk.lines().collect();
    let lines_len = lines.len();

    for idx in 0..lines_len - 1 {
        if lines[idx] != lines[idx + 1] {
            continue;
        }

        let mut mirror = true;
        let mut up = idx.checked_sub(1);
        let mut down = idx + 2;

        while mirror && up.is_some() && down < lines_len {
            let up_idx = up.unwrap();
            mirror = lines[up_idx] == lines[down];
            up = up_idx.checked_sub(1);
            down += 1;
        }

        if mirror {
            mirrors.push(idx + 1);
        }
    }

    mirrors
}

fn try_vertical(chunk: &str) -> Vec<usize> {
    let mut mirrors = Vec::new();
    let grid: Vec<&[u8]> = chunk.lines().map(|line| line.as_bytes()).collect();
    let width = grid[0].len();

    let check_cols =
        |left: usize, right: usize| (0..grid.len()).all(|row| grid[row][left] == grid[row][right]);

    for col in 0..width - 1 {
        if !check_cols(col, col + 1) {
            continue;
        }

        let mut mirror = true;
        let mut left = col.checked_sub(1);
        let mut right = col + 2;

        while mirror && left.is_some() && right < width {
            let left_idx = left.unwrap();
            mirror = check_cols(left_idx, right);
            left = left_idx.checked_sub(1);
            right += 1;
        }

        if mirror {
            mirrors.push(col + 1);
        }
    }

    mirrors
}

fn mirrors_sum<F>(input: &str, find_mirror: F) -> usize
where
    F: Fn(&str) -> Vec<MirrorLine> + Send + Sync,
{
    let chunks: Vec<_> = input.split("\n\n").collect();

    chunks
        .par_iter()
        .flat_map(|chunk| find_mirror(chunk))
        .map(MirrorLine::get_score)
        .sum()
}

fn part_1(input: &str) {
    let answer = mirrors_sum(input, find_mirror);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = mirrors_sum(input, find_smudged);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "13");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_solution() {
        assert_eq!(mirrors_sum(INPUT, find_mirror), 405);
        assert_eq!(mirrors_sum(INPUT, find_smudged), 400);
    }
}
