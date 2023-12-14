use crate::utls::read_text_from_file;

fn tilt_north(input: &str) -> Vec<Vec<u8>> {
    let mut cells: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    for row in 1..cells.len() {
        for col in 0..cells[0].len() {
            if cells[row][col] != b'O' {
                continue;
            }

            let mut curr_row = row;
            while let Some(r) = curr_row.checked_sub(1) {
                if cells[r][col] == b'.' {
                    cells[r][col] = b'O';
                    cells[curr_row][col] = b'.';
                    curr_row = r;
                } else {
                    break;
                }
            }
        }
    }

    cells
}

#[allow(unused)]
// For Debuging
fn print_cells(cells: &[Vec<u8>]) {
    cells.iter().for_each(|line| {
        line.iter().for_each(|ch| print!("{}", *ch as char));
        println!();
    });
}

fn calc_north_load(input: &str) -> usize {
    let titled = tilt_north(input);
    // print_cells(&titled);

    titled
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&&b| b == b'O').count() * (idx + 1))
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_north_load(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

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
        assert_eq!(calc_north_load(INPUT), 136);
    }
}

