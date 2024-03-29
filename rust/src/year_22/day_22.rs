use std::collections::BTreeMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
enum Inst {
    Move(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
struct Note {
    map: BTreeMap<Point, Tile>,
    instructions: Vec<Inst>,
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        let (map, inst) = value.split_once("\n\n").unwrap();

        let map: BTreeMap<Point, Tile> = map
            .lines()
            .enumerate()
            .flat_map(|(line_index, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| !ch.is_whitespace())
                    .map(move |(col_index, ch)| {
                        let tile = match ch {
                            '.' => Tile::Open,
                            '#' => Tile::Wall,
                            _ => panic!("invalid input"),
                        };
                        (Point::new(line_index + 1, col_index + 1), tile)
                    })
            })
            .collect();

        let inst_chars: Vec<char> = inst.chars().collect();

        let mut instructions = Vec::new();

        let mut index = 0;

        while index < inst_chars.len() {
            match inst_chars[index] {
                'L' => instructions.push(Inst::RotateLeft),
                'R' => instructions.push(Inst::RotateRight),
                n if n.is_ascii_digit() => {
                    let mut moving_index = index;
                    while moving_index < inst_chars.len()
                        && inst_chars[moving_index].is_ascii_digit()
                    {
                        moving_index += 1;
                    }

                    let num: usize = inst_chars[index..moving_index]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    index = moving_index - 1;
                    instructions.push(Inst::Move(num));
                }

                _ => unreachable!("invalid input"),
            }

            index += 1;
        }

        Note { map, instructions }
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug)]
struct State {
    pos: Point,
    facing: Facing,
}

impl State {
    fn new(pos: Point, facing: Facing) -> Self {
        Self { pos, facing }
    }
}

impl Inst {
    fn apply(&self, state: &mut State, map: &BTreeMap<Point, Tile>) {
        match self {
            Inst::RotateLeft => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Up => Facing::Left,
                }
            }
            Inst::RotateRight => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right,
                }
            }
            Inst::Move(num) => {
                for _ in 0..*num {
                    match state.facing {
                        Facing::Right => {
                            let mut point = state.pos;
                            point.col += 1;
                            if let Some(tile) = map.get(&point) {
                                match tile {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            } else {
                                let swap_col = *map
                                    .keys()
                                    .filter(|p| p.row == point.row)
                                    .map(|Point { row: _, col }| col)
                                    .min()
                                    .unwrap();
                                point.col = swap_col;
                                match map.get(&point).unwrap() {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            }
                        }
                        Facing::Down => {
                            let mut point = state.pos;
                            point.row += 1;
                            if let Some(tile) = map.get(&point) {
                                match tile {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            } else {
                                let swap_row = *map
                                    .keys()
                                    .filter(|p| p.col == point.col)
                                    .map(|Point { row, col: _ }| row)
                                    .min()
                                    .unwrap();
                                point.row = swap_row;
                                match map.get(&point).unwrap() {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            }
                        }
                        Facing::Left => {
                            let mut point = state.pos;
                            point.col -= 1;
                            if let Some(tile) = map.get(&point) {
                                match tile {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            } else {
                                let swap_col = *map
                                    .keys()
                                    .filter(|p| p.row == point.row)
                                    .map(|Point { row: _, col }| col)
                                    .max()
                                    .unwrap();
                                point.col = swap_col;
                                match map.get(&point).unwrap() {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            }
                        }
                        Facing::Up => {
                            let mut point = state.pos;
                            point.row -= 1;
                            if let Some(tile) = map.get(&point) {
                                match tile {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            } else {
                                let swap_row = *map
                                    .keys()
                                    .filter(|p| p.col == point.col)
                                    .map(|Point { row, col: _ }| row)
                                    .max()
                                    .unwrap();
                                point.row = swap_row;
                                match map.get(&point).unwrap() {
                                    Tile::Open => state.pos = point,
                                    Tile::Wall => break,
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //      1111122222
    //      1111122222
    //      1111122222
    //      1111122222
    //      33333
    //      33333
    //      33333
    //      33333
    //      33333
    // 5555544444
    // 5555544444
    // 5555544444
    // 5555544444
    // 5555544444
    // 66666
    // 66666
    // 66666
    // 66666
    // 66666

    fn apply_cube(&self, state: &mut State, map: &BTreeMap<Point, Tile>) {
        match self {
            Inst::RotateLeft => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Up => Facing::Left,
                }
            }
            Inst::RotateRight => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right,
                }
            }
            Inst::Move(num) => {
                for _ in 0..*num {
                    let mut point = state.pos;
                    let mut facing = state.facing;
                    match state.facing {
                        Facing::Right => {
                            // Easy path
                            if point.col % 50 != 0 {
                                point.col += 1;
                            } else {
                                // Cube calculations
                                match point.row {
                                    // 1 & 2
                                    (1..=50) => {
                                        match point.col {
                                            // 1 -> 2
                                            100 => {
                                                point.col += 1;
                                            }
                                            // 2 -> 4
                                            150 => {
                                                point.row = 151 - point.row;
                                                point.col = 100;
                                                facing = Facing::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 3 -> 2
                                    (51..=100) => {
                                        assert!((51..=100).contains(&point.col));
                                        point.col = point.row + 50;
                                        point.row = 50;
                                        facing = Facing::Up;
                                    }
                                    // 4 & 5
                                    (101..=150) => {
                                        match point.col {
                                            // 5 -> 4
                                            50 => {
                                                point.col += 1;
                                            }
                                            // 4 -> 2
                                            100 => {
                                                point.row = 151 - point.row;
                                                point.col = 150;
                                                facing = Facing::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 6 -> 4
                                    (151..=200) => {
                                        assert!((1..=51).contains(&point.col));
                                        point.col = point.row - 100;
                                        point.row = 150;
                                        facing = Facing::Up;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Facing::Left => {
                            // Easy path
                            if (point.col - 1) % 50 != 0 {
                                point.col -= 1;
                            } else {
                                // Cube calculations
                                match point.row {
                                    // 1 & 2
                                    (1..=50) => {
                                        match point.col {
                                            // 1 -> 5
                                            51 => {
                                                point.row = 151 - point.row;
                                                point.col = 1;
                                                facing = Facing::Right;
                                            }
                                            // 2 -> 1
                                            101 => {
                                                point.col -= 1;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 3 -> 5
                                    (51..=100) => {
                                        assert!((51..=100).contains(&point.col));
                                        point.col = point.row - 50;
                                        point.row = 101;
                                        facing = Facing::Down;
                                    }
                                    // 4 & 5
                                    (101..=150) => {
                                        match point.col {
                                            // 5 -> 1
                                            1 => {
                                                point.row = 151 - point.row;
                                                point.col = 51;
                                                facing = Facing::Right;
                                            }
                                            // 4 -> 5
                                            51 => {
                                                point.col -= 1;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 6 -> 1
                                    (151..=200) => {
                                        assert!((1..=51).contains(&point.col));
                                        point.col = point.row - 100;
                                        point.row = 1;
                                        facing = Facing::Down;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Facing::Down => {
                            // Easy path
                            if point.row % 50 != 0 {
                                point.row += 1;
                            } else {
                                // Cube calculations
                                match point.row {
                                    // 1 & 2
                                    50 => {
                                        match point.col {
                                            // 1 -> 3
                                            (51..=100) => {
                                                point.row += 1;
                                            }
                                            // 2 -> 3
                                            (101..=150) => {
                                                point.row = point.col - 50;
                                                point.col = 100;
                                                facing = Facing::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 3 -> 4
                                    100 => {
                                        assert!((51..=100).contains(&point.col));
                                        point.row += 1;
                                    }
                                    // 4 & 5
                                    150 => {
                                        match point.col {
                                            // 5 -> 6
                                            (1..=50) => {
                                                point.row += 1;
                                            }
                                            // 4 -> 6
                                            (51..=100) => {
                                                point.row = point.col + 100;
                                                point.col = 50;
                                                facing = Facing::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 6 -> 2
                                    200 => {
                                        assert!((1..=50).contains(&point.col));
                                        point.col += 100;
                                        point.row = 1;
                                        facing = Facing::Down;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Facing::Up => {
                            // Easy path
                            if (point.row - 1) % 50 != 0 {
                                point.row -= 1;
                            } else {
                                // Cube calculations
                                match point.row {
                                    // 1 & 2
                                    1 => {
                                        match point.col {
                                            // 1 -> 6
                                            (51..=100) => {
                                                point.row = point.col + 100;
                                                point.col = 1;
                                                facing = Facing::Right;
                                            }
                                            // 2 -> 6
                                            (101..=150) => {
                                                point.col -= 100;
                                                point.row = 200;
                                                facing = Facing::Up;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 3 -> 1
                                    51 => {
                                        assert!((51..=100).contains(&point.col));
                                        point.row -= 1;
                                    }
                                    // 4 & 5
                                    101 => {
                                        match point.col {
                                            // 5 -> 3
                                            (1..=50) => {
                                                point.row = point.col + 50;
                                                point.col = 51;
                                                facing = Facing::Right;
                                            }
                                            // 4 -> 3
                                            (51..=100) => {
                                                point.row -= 1;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // 6 -> 5
                                    151 => {
                                        assert!((1..=50).contains(&point.col));
                                        point.row -= 1;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }

                    match map.get(&point).unwrap_or_else(|| panic!("{:?}", &point)) {
                        Tile::Open => {
                            state.pos = point;
                            state.facing = facing;
                        }
                        Tile::Wall => break,
                    }
                }
            }
        }
    }
}

fn get_final_password(input: &str) -> usize {
    let note = Note::from(input);

    let mut state = State::new(Point::new(1, 51), Facing::Right);

    for ins in note.instructions.iter() {
        ins.apply(&mut state, &note.map);
    }

    1000 * state.pos.row + 4 * state.pos.col + state.facing as usize
}

fn get_final_password_cube(input: &str) -> usize {
    let note = Note::from(input);

    let mut state = State::new(Point::new(1, 51), Facing::Right);

    for ins in note.instructions.iter() {
        ins.apply_cube(&mut state, &note.map);
    }

    1000 * state.pos.row + 4 * state.pos.col + state.facing as usize
}

fn part_1() {
    let input = read_text_from_file("22", "22");
    let answer = get_final_password(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "22");
    let answer = get_final_password_cube(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"        ...#    
        .#..    
        #...    
        ....    
...#.......#    
........#...    
..#....#....    
..........#.    
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_() {
        assert_eq!(get_final_password(INPUT), 6032);
    }
}
