use crate::utls::{get_input_path, read_lines_from_file};

#[derive(Debug)]
enum Movement {
    Left,
    Up,
    Right,
    Down,
}

impl From<char> for Movement {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Movement::Left,
            'U' => Movement::Up,
            'R' => Movement::Right,
            'D' => Movement::Down,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Button {
    row: i8,
    col: i8,
}

impl Button {
    fn new(x: i8, y: i8) -> Self {
        Self { row: x, col: y }
    }

    fn get_num(&self) -> char {
        match (self.row, self.col) {
            (0, 0) => '1',
            (0, 1) => '2',
            (0, 2) => '3',
            (1, 0) => '4',
            (1, 1) => '5',
            (1, 2) => '6',
            (2, 0) => '7',
            (2, 1) => '8',
            (2, 2) => '9',
            _ => panic!("invalid state"),
        }
    }

    fn apply(&mut self, movement: Movement) {
        let increase = |val: &mut i8| {
            if *val < 2 {
                *val += 1;
            }
        };
        let decrease = |val: &mut i8| {
            if *val > 0 {
                *val -= 1;
            }
        };

        match movement {
            Movement::Left => decrease(&mut self.col),
            Movement::Up => decrease(&mut self.row),
            Movement::Right => increase(&mut self.col),
            Movement::Down => increase(&mut self.row),
        }
    }
}

fn get_code(lines: Vec<String>) -> String {
    let mut buttons = vec![];

    lines.iter().map(|line| line.chars()).for_each(|chars| {
        let mut button = buttons.last().unwrap_or(&Button::new(1, 1)).clone();
        for ch in chars {
            button.apply(ch.into());
        }
        buttons.push(button);
    });

    buttons.into_iter().map(|button| button.get_num()).collect()
}

fn part_1() {
    let lines = read_lines_from_file(&get_input_path("16", "02"));

    let key = get_code(lines);
    println!("pin is {key}");
}

#[derive(Clone)]
struct Button2(char);

impl Button2 {
    fn apply(&mut self, movement: Movement) {
        match movement {
            Movement::Left => match self.0 {
                '1' | '2' | '5' | 'A' | 'D' => {}
                '3' => self.0 = '2',
                '4' => self.0 = '3',
                '6' => self.0 = '5',
                '7' => self.0 = '6',
                '8' => self.0 = '7',
                '9' => self.0 = '8',
                'B' => self.0 = 'A',
                'C' => self.0 = 'B',
                _ => panic!("invalid state"),
            },
            Movement::Up => match self.0 {
                '1' | '2' | '5' | '4' | '9' => {}
                '3' => self.0 = '1',
                '6' => self.0 = '2',
                '7' => self.0 = '3',
                '8' => self.0 = '4',
                'A' => self.0 = '6',
                'B' => self.0 = '7',
                'C' => self.0 = '8',
                'D' => self.0 = 'B',
                _ => panic!("invalid state"),
            },
            Movement::Right => match self.0 {
                '1' | '4' | '9' | 'C' | 'D' => {}
                '2' => self.0 = '3',
                '3' => self.0 = '4',
                '5' => self.0 = '6',
                '6' => self.0 = '7',
                '7' => self.0 = '8',
                '8' => self.0 = '9',
                'A' => self.0 = 'B',
                'B' => self.0 = 'C',
                _ => panic!("invalid state"),
            },
            Movement::Down => match self.0 {
                '5' | 'A' | 'D' | 'C' | '9' => {}
                '1' => self.0 = '3',
                '2' => self.0 = '6',
                '3' => self.0 = '7',
                '4' => self.0 = '8',
                '6' => self.0 = 'A',
                '7' => self.0 = 'B',
                '8' => self.0 = 'C',
                'B' => self.0 = 'D',
                _ => panic!("invalid state"),
            },
        }
    }
}

fn get_code_2(lines: Vec<String>) -> String {
    let mut buttons = vec![];

    lines.iter().map(|line| line.chars()).for_each(|chars| {
        let mut button = buttons.last().unwrap_or(&Button2('5')).clone();
        for ch in chars {
            button.apply(ch.into());
        }
        buttons.push(button);
    });

    buttons.into_iter().map(|button| button.0).collect()
}

fn part_2() {
    let lines = read_lines_from_file(&get_input_path("16", "02"));

    let key = get_code_2(lines);
    println!("pin is {key}");
}
pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_code() {
        let lines = vec![
            String::from("ULL"),
            String::from("RRDDD"),
            String::from("LURDL"),
            String::from("UUUUD"),
        ];

        assert_eq!(get_code(lines.clone()), "1985");
        assert_eq!(get_code_2(lines), "5DB3");
    }
}
