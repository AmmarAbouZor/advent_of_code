use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    East,
    South,
    Empty,
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Cell::East,
            'v' => Cell::South,
            '.' => Cell::Empty,
            bad => unreachable!("Bad Input {bad}"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();

        Grid { cells }
    }
}

impl Grid {
    pub fn get_rounds_to_stop_moving(&mut self) -> usize {
        let mut rounds = 0;

        loop {
            rounds += 1;
            let available_east = self.get_cells_can_move_east();
            self.move_cells_east(&available_east);
            let available_south = self.get_cells_can_move_south();
            self.move_cells_south(&available_south);

            if available_east.is_empty() && available_south.is_empty() {
                return rounds;
            }
        }
    }

    fn get_cells_can_move_east(&self) -> Vec<(usize, usize)> {
        let rows = self.cells.len();
        let cols = self.cells[0].len();

        (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| {
                self.cells[row][col] == Cell::East
                    && self.cells[row][self.get_next_col(col)] == Cell::Empty
            })
            .collect()
    }

    #[inline]
    fn get_next_col(&self, col: usize) -> usize {
        let mut next_col = col + 1;
        if next_col == self.cells[0].len() {
            next_col = 0;
        }

        next_col
    }

    fn get_cells_can_move_south(&self) -> Vec<(usize, usize)> {
        let rows = self.cells.len();
        let cols = self.cells[0].len();

        (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| {
                self.cells[row][col] == Cell::South
                    && self.cells[self.get_next_row(row)][col] == Cell::Empty
            })
            .collect()
    }

    #[inline]
    fn get_next_row(&self, row: usize) -> usize {
        let mut next_row = row + 1;
        if next_row == self.cells.len() {
            next_row = 0;
        }

        next_row
    }

    fn move_cells_east(&mut self, cells: &[(usize, usize)]) {
        for &(row, col) in cells {
            let next_col = self.get_next_col(col);
            self.cells[row].swap(col, next_col);
        }
    }

    fn move_cells_south(&mut self, cells: &[(usize, usize)]) {
        for &(row, col) in cells {
            let next_row = self.get_next_row(row);
            self.cells[row][col] = Cell::Empty;
            self.cells[next_row][col] = Cell::South;
        }
    }
}

fn part_1() {
    let input = read_text_from_file("21", "25");
    let mut grid = Grid::from(input.as_str());

    let answer = grid.get_rounds_to_stop_moving();

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

    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_all() {
        let mut grid = Grid::from(INPUT);
        assert_eq!(grid.get_rounds_to_stop_moving(), 58);
    }
}

