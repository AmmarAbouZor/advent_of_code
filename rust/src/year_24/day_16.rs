use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    usize,
};

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
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next_pos(self, pos: &Pos) -> Pos {
        // coordinates should never go negative because of the borders.
        match self {
            Dir::North => Pos::new(pos.row.checked_sub(1).unwrap(), pos.col),
            Dir::East => Pos::new(pos.row, pos.col + 1),
            Dir::South => Pos::new(pos.row + 1, pos.col),
            Dir::West => Pos::new(pos.row, pos.col.checked_sub(1).unwrap()),
        }
    }

    fn rotate(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
}

impl State {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }
}

fn parse(input: &str) -> (Vec<Vec<Cell>>, Pos) {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(ridx, line)| {
            line.chars()
                .enumerate()
                .map(|(cidx, ch)| match ch {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'S' => {
                        start = Some(Pos::new(ridx, cidx));
                        Cell::Empty
                    }
                    'E' => Cell::Target,
                    invalid => panic!("{invalid}"),
                })
                .collect()
        })
        .collect();

    (grid, start.unwrap())
}

fn shorted_way(input: &str) -> usize {
    let (grid, start) = parse(input);
    let start_state = State::new(start, Dir::East);

    let mut states: HashMap<State, usize> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_state, 0));

    let mut shortes = usize::MAX;

    while let Some((mut state, score)) = queue.pop_front() {
        match states.entry(state) {
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get() <= &score {
                    continue;
                }
                occupied_entry.insert(score);
            }
            Entry::Vacant(vacant_entry) => _ = vacant_entry.insert(score),
        };

        for i in 0..4 {
            // Rotation can be clockwise or counterclockwise
            let factor = if i == 3 { 1 } else { i };

            let next = state.dir.next_pos(&state.pos);
            match grid[next.row][next.col] {
                Cell::Empty => {
                    let new_score = score + (factor * 1000) + 1;
                    queue.push_back((State::new(next, state.dir), new_score));
                }
                Cell::Wall => {}
                Cell::Target => {
                    let new_score = score + (factor * 1000) + 1;
                    shortes = shortes.min(new_score);
                }
            }
            state.dir = state.dir.rotate();
        }
    }

    shortes
}

fn part_1(input: &'static str) {
    let ans = shorted_way(input);
    println!("Part 1 answer is {ans}");
}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "16").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_solution() {
        let sum1 = shorted_way(INPUT_1);
        assert_eq!(sum1, 7036);

        let sum2 = shorted_way(INPUT_2);
        assert_eq!(sum2, 11048);
    }
}
