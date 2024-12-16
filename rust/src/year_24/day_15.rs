use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn next(self, pos: &Pos) -> Pos {
        // In this contest position can never negative because we are counting the border.
        match self {
            Dir::Up => Pos::new(pos.row.checked_sub(1).unwrap(), pos.col),
            Dir::Right => Pos::new(pos.row, pos.col + 1),
            Dir::Down => Pos::new(pos.row + 1, pos.col),
            Dir::Left => Pos::new(pos.row, pos.col.checked_sub(1).unwrap()),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Dir::Up),
            '>' => Ok(Dir::Right),
            'v' => Ok(Dir::Down),
            '<' => Ok(Dir::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Empty,
    Wall,
    Box,
    Robot,
}

impl From<char> for Slot {
    fn from(value: char) -> Self {
        match value {
            '.' => Slot::Empty,
            '#' => Slot::Wall,
            'O' => Slot::Box,
            '@' => Slot::Robot,
            invalid => unreachable!("{invalid}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotWiden {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

fn parse(input: &str) -> (Vec<Vec<Slot>>, Pos, Vec<Dir>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut start = None;
    let grid: Vec<Vec<Slot>> = map
        .lines()
        .enumerate()
        .map(|(ridx, row)| {
            row.chars()
                .enumerate()
                .map(|(cidx, char)| {
                    if start.is_none() && char == '@' {
                        start = Some(Pos::new(ridx, cidx));
                    }

                    char.into()
                })
                .collect()
        })
        .collect();

    let moves = moves
        .chars()
        .filter_map(|char| Dir::try_from(char).ok())
        .collect();

    (grid, start.unwrap(), moves)
}

fn apply_move(grid: &mut [Vec<Slot>], robot: Pos, dir: Dir) -> Pos {
    let mut to_move = vec![];
    let mut do_move = false;
    let mut current = robot;
    loop {
        let next = dir.next(&current);
        match grid[next.row][next.col] {
            Slot::Empty => {
                do_move = true;
                break;
            }
            Slot::Wall => break,
            Slot::Box => to_move.push(next),
            Slot::Robot => unreachable!("We have one robot only"),
        }
        current = next;
    }

    if !do_move {
        return robot;
    }

    for slot in to_move.iter().rev() {
        let next = dir.next(slot);
        grid[next.row][next.col] = grid[slot.row][slot.col];
    }

    let next_robot = dir.next(&robot);

    grid[robot.row][robot.col] = Slot::Empty;
    grid[next_robot.row][next_robot.col] = Slot::Robot;

    next_robot
}

fn sum_gps_coor(input: &str) -> usize {
    let (mut grid, mut robot, moves) = parse(input);

    for dir in moves {
        robot = apply_move(&mut grid, robot, dir);
    }

    let mut sum = 0;

    grid.iter().enumerate().for_each(|(ridx, row)| {
        row.iter().enumerate().for_each(|(cidx, slot)| {
            if matches!(slot, Slot::Box) {
                sum += (100 * ridx) + cidx
            }
        });
    });

    sum
}

#[allow(unused)]
fn print_grid(grid: &[Vec<Slot>]) {
    for row in grid {
        for item in row {
            let ch = match item {
                Slot::Empty => '.',
                Slot::Wall => '#',
                Slot::Box => 'O',
                Slot::Robot => '@',
            };
            print!("{ch}");
        }
        println!();
    }
}

fn part_1(input: &'static str) {
    let sum = sum_gps_coor(input);
    println!("Part 1 answer is {sum}");
}

fn widen_grid(grid: &[Vec<Slot>]) -> Vec<Vec<SlotWiden>> {
    use std::iter::repeat_n;
    let mut g = Vec::with_capacity(grid.len());
    for row in grid {
        let mut wide_row = Vec::with_capacity(row.len() * 2);
        for slot in row {
            match slot {
                Slot::Empty => wide_row.extend(repeat_n(SlotWiden::Empty, 2)),
                Slot::Wall => wide_row.extend(repeat_n(SlotWiden::Wall, 2)),
                Slot::Box => wide_row.extend([SlotWiden::BoxLeft, SlotWiden::BoxRight]),
                Slot::Robot => wide_row.extend([SlotWiden::Robot, SlotWiden::Empty]),
            }
        }
        g.push(wide_row);
    }

    g
}

fn apply_move_widen(grid: &mut [Vec<SlotWiden>], robot: Pos, dir: Dir) -> Pos {
    let horizontal = matches!(dir, Dir::Left | Dir::Right);
    if horizontal {
        let mut to_move = vec![];
        let mut do_move = false;
        let mut current = robot;
        loop {
            let next = dir.next(&current);
            match grid[next.row][next.col] {
                SlotWiden::Empty => {
                    do_move = true;
                    break;
                }
                SlotWiden::Wall => break,
                SlotWiden::BoxLeft | SlotWiden::BoxRight => to_move.push(next),
                SlotWiden::Robot => unreachable!("We have one robot only"),
            }
            current = next;
        }

        if !do_move {
            return robot;
        }

        for slot in to_move.iter().rev() {
            let next = dir.next(slot);
            grid[next.row][next.col] = grid[slot.row][slot.col];
        }

        let next_robot = dir.next(&robot);

        grid[robot.row][robot.col] = SlotWiden::Empty;
        grid[next_robot.row][next_robot.col] = SlotWiden::Robot;

        return next_robot;
    }

    // Vertical
    let mut to_bush = HashSet::from_iter([robot]);
    let mut to_move = Vec::new();

    let mut do_move = true;

    'outer: while !to_bush.is_empty() {
        let mut new_to_push = HashSet::new();
        for cur_pos in to_bush {
            let next_pos = dir.next(&cur_pos);
            let current = grid[cur_pos.row][cur_pos.col];
            let next = grid[next_pos.row][next_pos.col];
            match (current, next) {
                (SlotWiden::Empty, _) => {
                    panic!("Currnet can't be empty");
                }
                (SlotWiden::Wall, _) => panic!("Currnet can't be wall"),

                (_, SlotWiden::Wall) => {
                    do_move = false;
                    break 'outer;
                }

                (_, SlotWiden::Empty) => {
                    to_move.push(cur_pos);
                }

                (_, SlotWiden::Robot) => unreachable!(),

                (SlotWiden::BoxLeft, SlotWiden::BoxLeft) => {
                    new_to_push.insert(next_pos);
                    to_move.push(cur_pos);
                }
                (SlotWiden::BoxLeft, SlotWiden::BoxRight) => {
                    let left_pos = Pos::new(next_pos.row, next_pos.col.checked_sub(1).unwrap());
                    new_to_push.extend([left_pos, next_pos]);
                    to_move.push(cur_pos);
                }

                (SlotWiden::BoxRight, SlotWiden::BoxLeft) => {
                    let right_pos = Pos::new(next_pos.row, next_pos.col + 1);
                    new_to_push.extend([next_pos, right_pos]);
                    to_move.push(cur_pos);
                }
                (SlotWiden::BoxRight, SlotWiden::BoxRight) => {
                    new_to_push.insert(next_pos);
                    to_move.push(cur_pos);
                }

                (SlotWiden::Robot, SlotWiden::BoxLeft) => {
                    let right_pos = Pos::new(next_pos.row, next_pos.col + 1);
                    new_to_push.extend([next_pos, right_pos]);
                }
                (SlotWiden::Robot, SlotWiden::BoxRight) => {
                    let left_pos = Pos::new(next_pos.row, next_pos.col.checked_sub(1).unwrap());
                    new_to_push.extend([left_pos, next_pos]);
                }
            }
        }

        to_bush = new_to_push;
    }

    if !do_move {
        return robot;
    }

    for slot in to_move.iter().rev() {
        let next = dir.next(slot);
        let n = grid[next.row][next.col];
        grid[next.row][next.col] = grid[slot.row][slot.col];
        grid[slot.row][slot.col] = n;
    }

    let next_robot = dir.next(&robot);

    grid[robot.row][robot.col] = SlotWiden::Empty;
    grid[next_robot.row][next_robot.col] = SlotWiden::Robot;

    next_robot
}

fn sum_gps_widen(input: &str) -> usize {
    let (grid, _, moves) = parse(input);

    let mut grid = widen_grid(&grid);

    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(ridx, row)| {
            row.iter()
                .position(|&s| s == SlotWiden::Robot)
                .map(|col| Pos::new(ridx, col))
        })
        .unwrap();

    for dir in moves {
        robot = apply_move_widen(&mut grid, robot, dir);
    }

    let mut sum = 0;

    grid.iter().enumerate().for_each(|(ridx, row)| {
        row.iter().enumerate().for_each(|(cidx, slot)| {
            if matches!(slot, SlotWiden::BoxLeft) {
                sum += (100 * ridx) + cidx
            }
        });
    });

    sum
}

#[allow(unused)]
fn print_widen(grid: &[Vec<SlotWiden>]) {
    for row in grid {
        for item in row {
            let ch = match item {
                SlotWiden::Empty => '.',
                SlotWiden::Wall => '#',
                SlotWiden::BoxLeft => '[',
                SlotWiden::BoxRight => ']',
                SlotWiden::Robot => '@',
            };
            print!("{ch}");
        }
        println!();
    }
}

fn part_2(input: &'static str) {
    let ans = sum_gps_widen(input);
    println!("Part 2 answer is {ans}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "15").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_solution() {
        let sum = sum_gps_coor(INPUT_2);
        assert_eq!(sum, 2028);

        let sum = sum_gps_coor(INPUT);
        assert_eq!(sum, 10092);

        let sum_widen = sum_gps_widen(INPUT);
        assert_eq!(sum_widen, 9021);
    }
}
