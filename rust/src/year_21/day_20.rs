use std::fmt::Display;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Picture {
    pixels: Vec<Vec<bool>>,
}

const OFFSET: usize = 20;

impl From<&str> for Picture {
    fn from(input: &str) -> Self {
        // Make offset for the pixels so the picture can grow
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut pixels = vec![vec![false; width + 2 * OFFSET]; height + 2 * OFFSET];

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                let lid = char_to_bool(ch);
                pixels[row + OFFSET][col + OFFSET] = lid;
            })
        });

        Picture { pixels }
    }
}

impl Display for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let picture_txt: Vec<String> = self
            .pixels
            .iter()
            .map(|v| {
                v.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();

        let picture_txt = picture_txt.join("\n");

        write!(f, "{picture_txt}")
    }
}

impl Picture {
    fn apply_enhance(&mut self, enh_alg: &[bool], round: usize) {
        let mut px = self.pixels.clone();
        let width = self.pixels[0].len();
        let height = self.pixels.len();

        for row in OFFSET - round..height - OFFSET + round {
            for col in OFFSET - round..width - OFFSET + round {
                let mut grid = Vec::with_capacity(9);
                for r in row - 1..row + 2 {
                    for c in col - 1..col + 2 {
                        grid.push(self.pixels[r][c]);
                    }
                }
                // grid.extend_from_slice(&self.pixels[row - 2][col - 1..col + 2]);
                // grid.extend_from_slice(&self.pixels[row][col - 1..col + 2]);
                // grid.extend_from_slice(&self.pixels[row + 1][col - 1..col + 2]);
                let lid_idx = bool_slice_to_binary(&grid);
                px[row][col] = enh_alg[lid_idx];
            }
        }

        self.pixels = px;
    }

    fn get_lid_pixels(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&lid| lid).count()
    }
}

fn bool_slice_to_binary(arr: &[bool]) -> usize {
    // if arr.iter().all(|&b| !b) {
    //     println!("Hit");
    //     return 0;
    // }
    let mut result = 0;
    for &bit in arr {
        result <<= 1;
        if bit {
            result |= 1;
        }
    }

    result
}

#[inline]
fn char_to_bool(ch: char) -> bool {
    match ch {
        '.' => false,
        '#' => true,
        ch => unreachable!("Bad Input: '{ch}'"),
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Picture) {
    let (img_enh, picture) = input.split_once("\n\n").unwrap();

    let img_enh = img_enh
        .chars()
        .filter(|ch| *ch != ' ')
        .map(char_to_bool)
        .collect();
    let picture = Picture::from(picture);

    (img_enh, picture)
}

fn calc_lid_pixels(input: &str) -> usize {
    let (img_enh, mut picture) = parse_input(input);
    println!("{picture}");

    for i in 0..2 {
        picture.apply_enhance(&img_enh, i + 1);
    }

    println!("{picture}");

    picture.get_lid_pixels()
}

fn part_1() {
    let input = read_text_from_file("21", "20");
    let answer = calc_lid_pixels(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..## #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.### .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#. .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#..... .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.. ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#..... ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_lid_pixels(INPUT), 35);
    }
}

