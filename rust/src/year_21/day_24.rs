use std::collections::HashSet;

use crate::utls::read_text_from_file;

// This is borrowed from a solution found online
//
// Decompiling each block by hand we get:
//   00:  inp w      ; [I, 0, 0, Z]
//   01:  mul x 0
//   02:  add x z    ; [I, Z, 0, Z]
//   03:  mod x 26   ; [I, Z%26, 0, Z]
//   04:  div z <A>  ; [I, Z%26, 0, Z/A]
//   05:  add x <B>  ; [I, Z%26 + B, 0, Z/A]
//   06:  eql x w
//   07:  eql x 0    ; [I, (Z%26 + B) != I), 0, Z/A]
//   08:  mul y 0
//   09:  add y 25   ; [I, (Z%26 + B) != I, 25, Z/A]
//   10:  mul y x    ; [I, (Z%26 + B) != I, 25 * X, Z/A]
//   11:  add y 1    ; [I, (Z%26 + B) != I, 25 * X + 1, Z/A]
//   12:  mul z y    ; [I, (Z%26 + B) != I, 25 * X + 1, (Z/A) * (25 * X + 1)]
//   13:  mul y 0
//   14:  add y w    ; [I, (Z%26 + B) != I, I, (Z/A) * (25 * X + 1)]
//   15:  add y <C>  ; [I, (Z%26 + B) != I, I + C, (Z/A) * (25 * X + 1)]
//   16:  mul y x    ; [I, (Z%26 + B) != I, (I + C) * X, (Z/A) * (25 * X + 1)]
//   17:  add z y    ; [I, (Z%26 + B) != I, (I + C) * X, (Z/A) * (25 * X + 1) + (I + C) * X]
// => X = (Z%26 + B) != I
//    Z = (Z/A) * (25 * X + 1) + (I + C) * X
//
// Analyzing this formula, we see that z will never go to zero if at block b `z > 26^b`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    block_idx: usize,
    z: i64,
}

impl State {
    fn new(block_idx: usize, z: i64) -> Self {
        Self { block_idx, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// This represents the changing variables on each iteration
/// See the Decompiling comment for more details
struct Block {
    a: i64,
    b: i64,
    c: i64,
}

impl Block {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }
}

fn find_model_num(
    cache: &mut HashSet<State>,
    blocks: &[Block],
    target_rng: &[i64],
    block_idx: usize,
    z: i64,
) -> Option<i64> {
    if block_idx == blocks.len() {
        return match z {
            0 => Some(0),
            _ => None,
        };
    }

    if z > 26i64.pow(14 - block_idx as u32) {
        return None;
    }

    if cache.contains(&State::new(block_idx, z)) {
        return None;
    }

    let block = blocks.get(block_idx).unwrap();

    for &i in target_rng {
        let new_z = if z % 26 + block.b == i {
            z / block.a
        } else {
            (z / block.a) * 26 + i + block.c
        };

        if let Some(n) = find_model_num(cache, blocks, target_rng, block_idx + 1, new_z) {
            return Some(i * 10i64.pow(13 - block_idx as u32) + n);
        }
    }

    cache.insert(State::new(block_idx, z));

    None
}

fn find_max_valid(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();

    let blocks: Vec<_> = lines
        .chunks(18)
        .map(|chunk| {
            let a = chunk[4][6..].parse().unwrap();
            let b = chunk[5][6..].parse().unwrap();
            let c = chunk[15][6..].parse().unwrap();

            Block::new(a, b, c)
        })
        .collect();

    let mut cache = HashSet::new();

    find_model_num(&mut cache, &blocks, &[9, 8, 7, 6, 5, 4, 3, 2, 1], 0, 0).unwrap()
}

fn find_min_valid(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();

    let blocks: Vec<_> = lines
        .chunks(18)
        .map(|chunk| {
            let a = chunk[4][6..].parse().unwrap();
            let b = chunk[5][6..].parse().unwrap();
            let c = chunk[15][6..].parse().unwrap();

            Block::new(a, b, c)
        })
        .collect();

    let mut cache = HashSet::new();

    find_model_num(&mut cache, &blocks, &[1, 2, 3, 4, 5, 6, 7, 8, 9], 0, 0).unwrap()
}

fn part_1() {
    let input = read_text_from_file("21", "24");
    let answer = find_max_valid(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "24");
    let answer = find_min_valid(input.as_str());

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}
