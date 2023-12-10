use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Maze {
    cells: Vec<Vec<char>>,
    start: Pos,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let mut start = None;
        let cells = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        if start.is_none() && ch == 'S' {
                            start = Some(Pos::new(row, col));
                        }
                        ch
                    })
                    .collect()
            })
            .collect();

        Maze {
            cells,
            start: start.unwrap(),
        }
    }
}

impl Maze {
    fn calc_steps(&self) -> usize {
        let starts = vec![
            // Up
            (
                'L',
                self.start,
                Pos::new(self.start.row, self.start.col + 1),
            ),
            // Down
            (
                'F',
                self.start,
                Pos::new(self.start.row, self.start.col + 1),
            ),
            // Left
            (
                '7',
                self.start,
                Pos::new(self.start.row + 1, self.start.col),
            ),
            // Right
            (
                'F',
                self.start,
                Pos::new(self.start.row + 1, self.start.col),
            ),
        ];

        for (mut ch, mut curr, mut prev) in starts {
            let mut count = 0;
            while let Some(new_pos) = self.next_valid_pos(ch, &prev, &curr) {
                count += 1;
                if new_pos == self.start {
                    return count / 2;
                }
                ch = self.get_char(&new_pos).unwrap();
                prev = curr;
                curr = new_pos;
            }
        }

        unreachable!("Must be solved before this point");
    }

    fn next_valid_pos(&self, cur_char: char, prev_pos: &Pos, curr_pos: &Pos) -> Option<Pos> {
        match cur_char {
            '|' => {
                // going up
                if prev_pos.row > curr_pos.row {
                    let next_pos = Pos::new(curr_pos.row.wrapping_sub(1), curr_pos.col);
                    self.get_valid_up(next_pos)
                }
                // going down
                else {
                    let next_pos = Pos::new(curr_pos.row + 1, curr_pos.col);
                    self.get_valid_down(next_pos)
                }
            }
            '-' => {
                // going Left
                if prev_pos.col > curr_pos.col {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col.wrapping_sub(1));
                    self.get_valid_left(next_pos)
                }
                // going right
                else {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col + 1);
                    self.get_valid_right(next_pos)
                }
            }

            'L' => {
                // going up
                if prev_pos.row == curr_pos.row {
                    let next_pos = Pos::new(curr_pos.row.wrapping_sub(1), curr_pos.col);
                    self.get_valid_up(next_pos)
                }
                // going right
                else {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col + 1);
                    self.get_valid_right(next_pos)
                }
            }
            'J' => {
                // going up
                if prev_pos.row == curr_pos.row {
                    let next_pos = Pos::new(curr_pos.row.wrapping_sub(1), curr_pos.col);
                    self.get_valid_up(next_pos)
                }
                // going left
                else {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col.wrapping_sub(1));
                    self.get_valid_left(next_pos)
                }
            }
            '7' => {
                // going down
                if prev_pos.row == curr_pos.row {
                    let next_pos = Pos::new(curr_pos.row + 1, curr_pos.col);
                    self.get_valid_down(next_pos)
                }
                // going left
                else {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col.wrapping_sub(1));
                    self.get_valid_left(next_pos)
                }
            }
            'F' => {
                // going down
                if prev_pos.row == curr_pos.row {
                    let next_pos = Pos::new(curr_pos.row + 1, curr_pos.col);
                    self.get_valid_down(next_pos)
                }
                // going right
                else {
                    let next_pos = Pos::new(curr_pos.row, curr_pos.col + 1);
                    self.get_valid_right(next_pos)
                }
            }
            '.' => None,
            invalid => unreachable!("Invaid char: '{invalid}'"),
        }
    }

    fn get_char(&self, pos: &Pos) -> Option<char> {
        self.cells
            .get(pos.row)
            .and_then(|row| row.get(pos.col).cloned())
    }

    fn get_valid_up(&self, next_pos: Pos) -> Option<Pos> {
        match self.get_char(&next_pos) {
            Some('|' | 'F' | '7' | 'S') => Some(next_pos),
            _ => None,
        }
    }

    fn get_valid_down(&self, next_pos: Pos) -> Option<Pos> {
        match self.get_char(&next_pos) {
            Some('|' | 'L' | 'J' | 'S') => Some(next_pos),
            _ => None,
        }
    }

    fn get_valid_left(&self, next_pos: Pos) -> Option<Pos> {
        match self.get_char(&next_pos) {
            Some('-' | 'L' | 'F' | 'S') => Some(next_pos),
            _ => None,
        }
    }

    fn get_valid_right(&self, next_pos: Pos) -> Option<Pos> {
        match self.get_char(&next_pos) {
            Some('-' | 'J' | '7' | 'S') => Some(next_pos),
            _ => None,
        }
    }
}

fn calc_steps(input: &str) -> usize {
    let maze = Maze::from(input);
    maze.calc_steps()
}

fn part_1(input: &str) {
    let answer = calc_steps(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "10");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_solution() {
        assert_eq!(calc_steps(INPUT_1), 4);
        assert_eq!(calc_steps(INPUT_2), 8);
    }
}

