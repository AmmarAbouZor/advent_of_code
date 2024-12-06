use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn next(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.wrapping_sub(1)),
        }
    }
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            invalid => unreachable!("{invalid}"),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn calc_visited(input: &str) -> usize {
    let grid = parse(input);
    let (mut cur_pos, mut dir) = grid
        .iter()
        .enumerate()
        .find_map(|(row, vec)| {
            vec.iter().enumerate().find_map(|(col, char)| {
                (char == &'^').then(|| {
                    let pos = (row, col);
                    let dir = Direction::from(*char);
                    (pos, dir)
                })
            })
        })
        .unwrap();

    let mut visited = HashSet::new();

    loop {
        visited.insert(cur_pos);

        let next_pos = dir.next(cur_pos.0, cur_pos.1);
        let next_char = match grid.get(next_pos.0).and_then(|row| row.get(next_pos.1)) {
            Some(ch) => ch,
            None => break,
        };
        dir = match next_char {
            '#' => dir.rotate(),
            _ => dir,
        };
        cur_pos = dir.next(cur_pos.0, cur_pos.1);
    }

    visited.len()
}

fn part_1(input: &'static str) {
    let visited = calc_visited(input);
    println!("Part 1 answer is {visited}");
}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "06").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_solution() {
        let visited = calc_visited(INPUT);
        assert_eq!(visited, 41);
    }
}

