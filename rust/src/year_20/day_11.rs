use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for State {
    fn from(ch: char) -> Self {
        match ch {
            'L' => State::Empty,
            '#' => State::Occupied,
            '.' => State::Floor,
            invalid => unreachable!("Bad input: '{invalid}'"),
        }
    }
}

impl State {
    fn get_char(&self) -> char {
        match self {
            State::Empty => 'L',
            State::Occupied => '#',
            State::Floor => '.',
        }
    }
}

#[derive(Debug)]
struct SeatLayout {
    cells: Vec<Vec<State>>,
}

impl From<&str> for SeatLayout {
    fn from(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| line.chars().map(State::from).collect())
            .collect();

        SeatLayout { cells }
    }
}

impl SeatLayout {
    fn apply_round_surround(&mut self) -> bool {
        let mut changed = false;

        let rows_count = self.cells.len();
        let cols_count = self.cells[0].len();

        let mut new_layout = vec![vec![State::Floor; cols_count]; rows_count];

        for row in 0..rows_count {
            for col in 0..cols_count {
                new_layout[row][col] = match self.cells[row][col] {
                    State::Empty => {
                        let all_empty = (row.saturating_sub(1)..=row + 1)
                            .flat_map(|r| (col.saturating_sub(1)..=col + 1).map(move |c| (r, c)))
                            .all(|(r, c)| {
                                let state =
                                    self.cells.get(r).and_then(|row_slice| row_slice.get(c));

                                !matches!(state, Some(State::Occupied))
                            });
                        if all_empty {
                            changed = true;
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        let occupied_count = (row.saturating_sub(1)..=row + 1)
                            .flat_map(|r| (col.saturating_sub(1)..=col + 1).map(move |c| (r, c)))
                            .filter(|&(r, c)| {
                                self.cells.get(r).is_some_and(|row_slice| {
                                    row_slice
                                        .get(c)
                                        .is_some_and(|&state| matches!(state, State::Occupied))
                                })
                            })
                            .count();

                        // Count the current cell too
                        if occupied_count >= 5 {
                            changed = true;
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    State::Floor => State::Floor,
                }
            }
        }

        self.cells = new_layout;

        changed
    }

    fn apply_round_visible(&mut self) -> bool {
        let mut changed = false;

        let rows_count = self.cells.len();
        let cols_count = self.cells[0].len();

        let mut new_layout = vec![vec![State::Floor; cols_count]; rows_count];

        const DELTAS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for row in 0..rows_count {
            for col in 0..cols_count {
                new_layout[row][col] = match self.cells[row][col] {
                    State::Empty => {
                        let all_empty = DELTAS
                            .iter()
                            .map(|(d_row, d_col)| self.find_first_seat(row, col, *d_row, *d_col))
                            .all(|state| !matches!(state, Some(State::Occupied)));
                        if all_empty {
                            changed = true;
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        let occupied_count = DELTAS
                            .iter()
                            .map(|(d_row, d_col)| self.find_first_seat(row, col, *d_row, *d_col))
                            .filter(|state| matches!(state, Some(State::Occupied)))
                            .count();

                        if occupied_count >= 5 {
                            changed = true;
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    State::Floor => State::Floor,
                }
            }
        }

        self.cells = new_layout;

        changed
    }

    fn find_first_seat(
        &self,
        row: usize,
        col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> Option<State> {
        let mut row = row as isize;
        let mut col = col as isize;

        let rows_count = self.cells.len() as isize;
        let cols_count = self.cells[0].len() as isize;

        while (0..rows_count).contains(&row) && (0..cols_count).contains(&col) {
            row += delta_row;
            col += delta_col;
            match self
                .cells
                .get(row as usize)
                .and_then(|row_slice| row_slice.get(col as usize))
            {
                Some(State::Occupied) => return Some(State::Occupied),
                Some(State::Empty) => return Some(State::Empty),
                _ => {}
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print(&self) {
        let lines: Vec<String> = self
            .cells
            .iter()
            .map(|row| row.iter().map(State::get_char).collect::<String>())
            .collect();

        for line in lines {
            println!("{line}");
        }
    }

    fn get_occupied_count(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|state| matches!(state, State::Occupied))
            .count()
    }
}

fn calc_occupied_surround(input: &str) -> usize {
    let mut layout = SeatLayout::from(input);
    let mut counter = 0;
    while layout.apply_round_surround() {
        counter += 1;
    }

    println!("Rounds count is {counter}");

    layout.get_occupied_count()
}

fn calc_occupied_visible(input: &str) -> usize {
    let mut layout = SeatLayout::from(input);
    let mut counter = 0;
    while layout.apply_round_visible() {
        counter += 1;
    }

    println!("Rounds count is {counter}");

    layout.get_occupied_count()
}

fn part_1(input: &str) {
    let answer = calc_occupied_surround(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_occupied_visible(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("20", "11");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part() {
        assert_eq!(calc_occupied_surround(INPUT), 37);
        assert_eq!(calc_occupied_visible(INPUT), 26);
    }
}
