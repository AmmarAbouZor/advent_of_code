use rayon::{iter::ParallelIterator, slice::ParallelSlice};

fn to_grid(input: &'static str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn calc_slice(slice: &[char]) -> usize {
    slice
        .par_windows(4)
        .filter(|w| w == &['X', 'M', 'A', 'S'] || w == &['S', 'A', 'M', 'X'])
        .count()
}

fn calc_all(input: &'static str) -> usize {
    let grid = to_grid(input);
    let mut answer = 0;

    // Lines:
    answer += grid.iter().map(|line| calc_slice(line)).sum::<usize>();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // Reuse the same vector to avoid collecting memory on each iteration
    let mut vec = Vec::with_capacity(num_rows);

    // Columns:
    for col in 0..num_cols {
        vec.clear();
        for row in &grid {
            vec.push(row[col]);
        }
        answer += calc_slice(&vec);
    }

    // Primary Diagonal:
    // ###
    // .##
    // ..#
    for start in 0..num_cols {
        vec.clear();
        let mut row = 0;
        let mut col = start;
        while row < num_rows && col < num_cols {
            vec.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        answer += calc_slice(&vec);
    }
    // ...
    // #..
    // ##.
    for start in 1..num_rows {
        vec.clear();
        let mut row = start;
        let mut col = 0;
        while row < num_rows && col < num_cols {
            vec.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        answer += calc_slice(&vec);
    }

    // Secondary Diagonal
    // ###
    // ##.
    // #..
    for start in (0..num_cols).rev() {
        vec.clear();
        let mut row = 0;
        let mut col = start;
        while row < num_rows {
            vec.push(grid[row][col]);
            if col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        answer += calc_slice(&vec);
    }

    // ...
    // ..#
    // .##
    for start in 1..num_rows {
        vec.clear();
        let mut row = start;
        let mut col = num_cols - 1;
        while row < num_rows {
            vec.push(grid[row][col]);
            if col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        answer += calc_slice(&vec);
    }

    answer
}

#[allow(unused)]
fn dbg_print(slice: &[char]) {
    let word: String = slice.iter().collect();
    println!("{word}")
}

fn part_1(input: &'static str) {
    let calc = calc_all(input);
    println!("Part 1 answer is {calc}")
}

fn is_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    // Center must inside the margin of one
    if row == 0 || row > grid.len() - 2 {
        return false;
    }

    if col == 0 || col > grid[0].len() - 2 {
        return false;
    }

    if grid[row][col] != 'A' {
        return false;
    }

    let left_top = grid[row - 1][col - 1];
    let right_top = grid[row - 1][col + 1];
    let left_bottom = grid[row + 1][col - 1];
    let right_bottom = grid[row + 1][col + 1];

    let inside_primary =
        (left_top == 'M' && right_bottom == 'S') || (left_top == 'S' && right_bottom == 'M');

    if !inside_primary {
        return false;
    }

    // Inside Secondary
    (right_top == 'M' && left_bottom == 'S') || (right_top == 'S' && left_bottom == 'M')
}

fn calc_x_mas(input: &'static str) -> usize {
    let grid = to_grid(input);
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut answer = 0;
    for row in 1..num_rows - 1 {
        for col in 1..num_cols - 1 {
            if is_x_mas(&grid, row, col) {
                answer += 1;
            }
        }
    }

    answer
}

fn part_2(input: &'static str) {
    let x_mas = calc_x_mas(input);
    println!("Part 2 answer is {x_mas}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "04").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_solution() {
        let calc = calc_all(INPUT);
        assert_eq!(calc, 18);

        let x_mas = calc_x_mas(INPUT);
        assert_eq!(x_mas, 9)
    }
}
