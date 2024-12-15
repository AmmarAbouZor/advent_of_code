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
        grid[next.row][next.col] = grid[current.row][current.col];
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
        // print_grid(&grid);
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

fn part_2(input: &'static str) {}

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
    }
}

