use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Dir {
    North,
    West,
    South,
    East,
}

impl Dir {
    fn next(self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }
}

fn tilt_north(cells: &mut [Vec<u8>]) {
    for row in 1..cells.len() {
        for col in 0..cells[0].len() {
            if cells[row][col] != b'O' {
                continue;
            }

            let mut curr_row = row;
            while let Some(prev_row) = curr_row.checked_sub(1) {
                if cells[prev_row][col] == b'.' {
                    cells[prev_row][col] = b'O';
                    cells[curr_row][col] = b'.';
                    curr_row = prev_row;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_south(cells: &mut [Vec<u8>]) {
    for row in (0..cells.len() - 1).rev() {
        for col in 0..cells[0].len() {
            if cells[row][col] != b'O' {
                continue;
            }

            let mut curr_row = row;
            while curr_row < cells.len() - 1 {
                let next_row = curr_row + 1;
                if cells[next_row][col] == b'.' {
                    cells[next_row][col] = b'O';
                    cells[curr_row][col] = b'.';
                    curr_row = next_row;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_west(cells: &mut [Vec<u8>]) {
    for row in 0..cells.len() {
        for col in 1..cells[0].len() {
            if cells[row][col] != b'O' {
                continue;
            }

            let mut curr_col = col;
            while let Some(prev_col) = curr_col.checked_sub(1) {
                if cells[row][prev_col] == b'.' {
                    cells[row][prev_col] = b'O';
                    cells[row][curr_col] = b'.';
                    curr_col = prev_col;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_east(cells: &mut [Vec<u8>]) {
    for row in 0..cells.len() {
        for col in (0..cells[0].len() - 1).rev() {
            if cells[row][col] != b'O' {
                continue;
            }

            let mut curr_col = col;
            while curr_col < cells[0].len() - 1 {
                let next_col = curr_col + 1;
                if cells[row][next_col] == b'.' {
                    cells[row][next_col] = b'O';
                    cells[row][curr_col] = b'.';
                    curr_col = next_col;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_cycles(cells: &mut [Vec<u8>]) {
    let mut dir = Dir::North;
    // for _ in 0..1000000000 {
    // it settle down after 1000 in my input
    for _ in 0..1000 {
        for _ in 0..4 {
            match dir {
                Dir::North => tilt_north(cells),
                Dir::West => tilt_west(cells),
                Dir::South => tilt_south(cells),
                Dir::East => tilt_east(cells),
            }
            dir = dir.next();
        }
    }
}

#[allow(unused)]
// For Debuging
fn print_cells(cells: &[Vec<u8>]) {
    cells.iter().for_each(|line| {
        line.iter().for_each(|ch| print!("{}", *ch as char));
        println!();
    });
}

fn calc_load<F>(input: &str, tilt: F) -> usize
where
    F: Fn(&mut [Vec<u8>]),
{
    let mut cells: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    tilt(&mut cells);

    cells
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&&b| b == b'O').count() * (idx + 1))
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_load(input, tilt_north);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_load(input, tilt_cycles);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "14");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_solution() {
        assert_eq!(calc_load(INPUT, tilt_north), 136);
        assert_eq!(calc_load(INPUT, tilt_cycles), 64);
    }
}
