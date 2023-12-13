use crate::utls::read_text_from_file;

#[derive(Debug)]
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

fn find_mirror(chunk: &str) -> MirrorLine {
    match try_horizontal(chunk) {
        Some(hor) => MirrorLine::Horizontal(hor),
        None => try_vertical(chunk)
            .map(MirrorLine::Vertical)
            .expect("Each chunk must have a mirror line"),
    }
}

fn try_horizontal(chunk: &str) -> Option<usize> {
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
            return Some(idx + 1);
        }
    }

    None
}

fn try_vertical(chunk: &str) -> Option<usize> {
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
            return Some(col + 1);
        }
    }

    None
}

fn mirrors_sum(input: &str) -> usize {
    input
        .split("\n\n")
        .map(find_mirror)
        .map(MirrorLine::get_score)
        .sum()
}

fn part_1(input: &str) {
    let answer = mirrors_sum(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

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
        assert_eq!(mirrors_sum(INPUT), 405);
    }
}

