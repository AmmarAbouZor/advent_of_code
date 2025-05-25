use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u8>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let cells = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        Grid { cells }
    }
}

impl Grid {
    fn count_trees(&self, drow: usize, dcol: usize) -> usize {
        let mut count = 0;
        let (mut row, mut col) = (0, 0);
        let col_len = self.cells[0].len();
        while row < self.cells.len() {
            if self.cells[row][col % col_len] == b'#' {
                count += 1;
            }
            row += drow;
            col += dcol;
        }
        count
    }
}

fn calc_slops_prod(input: &str) -> usize {
    let grid = Grid::from(input);
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes
        .into_iter()
        .map(|(drow, dcol)| grid.count_trees(drow, dcol))
        .product()
}

fn part_1() {
    let input = read_text_from_file("20", "03");
    let grid = Grid::from(input.as_str());
    let answer = grid.count_trees(1, 3);

    println!("Part 1 answer is {answer}");
}
fn part_2() {
    let input = read_text_from_file("20", "03");
    let answer = calc_slops_prod(input.as_str());

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        let grid = Grid::from(INPUT);
        assert_eq!(grid.count_trees(1, 3), 7);
        assert_eq!(calc_slops_prod(INPUT), 336);
    }
}
