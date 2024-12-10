use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> &'static [Dir] {
        &[Dir::Up, Dir::Right, Dir::Down, Dir::Left]
    }

    fn next(self, pos: &Pos) -> Pos {
        match self {
            Dir::Up => Pos::new(pos.row.saturating_sub(1), pos.col),
            Dir::Right => Pos::new(pos.row, pos.col + 1),
            Dir::Down => Pos::new(pos.row + 1, pos.col),
            Dir::Left => Pos::new(pos.row, pos.col.saturating_sub(1)),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

fn calc_score(input: &str, unique: bool) -> usize {
    let grid = parse(input);
    grid.iter()
        .enumerate()
        .flat_map(|(ri, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &num)| num == 0)
                .map(move |(ci, _)| Pos::new(ri, ci))
        })
        .map(|start| calc_valid_routes(&grid, start, unique))
        .sum()
}

fn calc_valid_routes(grid: &[Vec<i8>], start: Pos, unique: bool) -> usize {
    let mut queue = vec![(start, 0)];
    let mut visited = HashSet::new();
    let mut count = 0;
    while let Some((pos, current_score)) = queue.pop() {
        for dir in Dir::all() {
            let next_pos = dir.next(&pos);
            let next_score = match grid.get(next_pos.row).and_then(|row| row.get(next_pos.col)) {
                Some(next_score) => *next_score,
                None => continue,
            };

            if next_score - current_score == 1 {
                if next_score == 9 {
                    if !unique && !visited.insert(next_pos) {
                        continue;
                    }

                    count += 1;
                } else {
                    queue.push((next_pos, next_score));
                }
            }
        }
    }

    count
}

fn part_1(input: &'static str) {
    let ans = calc_score(input, false);
    println!("Part 1 answer is {ans}")
}

fn part_2(input: &'static str) {
    let ans = calc_score(input, true);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "10").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_solution() {
        let sum = calc_score(INPUT, false);
        assert_eq!(sum, 36);

        let sum_unique = calc_score(INPUT, true);
        assert_eq!(sum_unique, 81);
    }
}
