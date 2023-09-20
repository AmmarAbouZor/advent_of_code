use std::fmt::Display;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Picture {
    pixels: Vec<Vec<bool>>,
}

impl From<&str> for Picture {
    fn from(input: &str) -> Self {
        // Make offset for the pixels so the picture can grow
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut pixels = vec![vec![false; width]; height];

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                let lid = char_to_bool(ch);
                pixels[row][col] = lid;
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
        let new_width = self.pixels[0].len() + 2;
        let new_height = self.pixels.len() + 2;
        let mut px = vec![vec![false; new_width]; new_height];

        for (row_idx, row) in px.iter_mut().enumerate() {
            for (col_idx, val) in row.iter_mut().enumerate() {
                let mut grid = Vec::with_capacity(9);
                for r in row_idx..row_idx + 3 {
                    for c in col_idx..col_idx + 3 {
                        grid.push(
                            *self
                                .pixels
                                .get(r.wrapping_sub(2))
                                .and_then(|row| row.get(c.wrapping_sub(2)))
                                .unwrap_or(&(round & 1 == 1)),
                        );
                    }
                }
                let lid_idx = bool_slice_to_binary(&grid);
                *val = enh_alg[lid_idx];
            }
        }

        self.pixels = px;
    }

    fn get_lid_pixels(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&lid| lid).count()
    }
}

fn bool_slice_to_binary(arr: &[bool]) -> usize {
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

fn calc_lid_pixels(input: &str, target: usize) -> usize {
    let (img_enh, mut picture) = parse_input(input);
    println!("{picture}");

    for i in 0..target {
        picture.apply_enhance(&img_enh, i);
    }

    picture.get_lid_pixels()
}

fn part_1() {
    let input = read_text_from_file("21", "20");
    let answer = calc_lid_pixels(input.as_str(), 2);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "20");
    let answer = calc_lid_pixels(input.as_str(), 50);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}
