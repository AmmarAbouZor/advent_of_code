use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    fn next(self, cor: &Pos) -> Pos {
        // No negative numbers because we have borders
        match self {
            Dir::Up => Pos::new(cor.row - 1, cor.col),
            Dir::Right => Pos::new(cor.row, cor.col + 1),
            Dir::Down => Pos::new(cor.row + 1, cor.col),
            Dir::Left => Pos::new(cor.row, cor.col - 1),
        }
    }
}

#[derive(Debug)]
struct StartInfo {
    walls: HashSet<Pos>,
    start: Pos,
    end: Pos,
    rows_count: usize,
    cols_count: usize,
}

/// walls
fn parse(input: &str) -> StartInfo {
    let grid: Vec<Vec<_>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let rows_count = grid.len();
    let cols_count = grid[0].len();
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (ridx, row) in grid.iter().enumerate() {
        for (cidx, &ch) in row.iter().enumerate() {
            match ch {
                '#' => _ = walls.insert(Pos::new(ridx, cidx)),
                'S' => start = Some(Pos::new(ridx, cidx)),
                'E' => end = Some(Pos::new(ridx, cidx)),
                _ => {}
            }
        }
    }

    StartInfo {
        walls,
        rows_count,
        cols_count,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn shortest_path(start: &Pos, end: &Pos, walls: &HashSet<Pos>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    let mut shortest = usize::MAX;
    while let Some((pos, score)) = queue.pop_front() {
        if &pos == end {
            shortest = shortest.min(score);
            continue;
        }

        match visited.entry(pos) {
            Entry::Occupied(mut occupied_entry) => {
                if *occupied_entry.get() > score {
                    _ = occupied_entry.insert(score)
                } else {
                    continue;
                }
            }
            Entry::Vacant(vacant_entry) => _ = vacant_entry.insert(score),
        }

        for dir in Dir::all() {
            let next = dir.next(&pos);
            if !walls.contains(&next) {
                queue.push_back((next, score + 1));
            }
        }
    }

    shortest
}

/// Map<diff, counts>
fn cheat_counts(input: &str) -> HashMap<usize, usize> {
    let StartInfo {
        walls,
        start,
        end,
        rows_count,
        cols_count,
    } = parse(input);
    let original = shortest_path(&start, &end, &walls);

    let mut walls_to_remove = Vec::new();
    for row in 1..rows_count - 1 {
        for col in 1..cols_count - 1 {
            let pos = Pos::new(row, col);
            if !walls.contains(&pos) {
                continue;
            }

            if Dir::all()
                .iter()
                .map(|dir| dir.next(&pos))
                .filter(|n| !walls.contains(n))
                .count()
                > 1
            {
                walls_to_remove.push(pos);
            }
        }
    }

    let dists: Vec<_> = walls_to_remove
        .par_iter()
        .map(|wall| {
            let mut walls = walls.clone();
            assert!(walls.remove(wall));
            let shortest = shortest_path(&start, &end, &walls);
            original.checked_sub(shortest).unwrap()
        })
        .collect();

    dists.into_iter().counts()
}

fn part_1(input: &'static str) {
    // My initial solution
    let counts = cheat_counts(input);
    let sum: usize = counts
        .iter()
        .filter(|(&diff, _)| diff >= 100)
        .map(|(_, c)| c)
        .sum();

    println!("Part 1 answer is {sum}");

    // Solution after finding out about Manhattan distance solution
    let ans = solve_with_manhattan_dist(input, 2, 100);
    println!("Part 1 answer with manhattan dist {ans}");
}

fn part_2(input: &'static str) {
    let ans = solve_with_manhattan_dist(input, 20, 100);
    println!("Part 2 answer is {ans}");
}

/// Get score for all positions in the map using bfs algorithm
fn positions_scores(start: &Pos, end: &Pos, walls: &HashSet<Pos>) -> HashMap<Pos, usize> {
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    while let Some((pos, score)) = queue.pop_front() {
        match visited.entry(pos) {
            Entry::Occupied(mut occupied_entry) => {
                if *occupied_entry.get() > score {
                    _ = occupied_entry.insert(score)
                } else {
                    continue;
                }
            }
            Entry::Vacant(vacant_entry) => _ = vacant_entry.insert(score),
        }

        if &pos == end {
            continue;
        }

        for dir in Dir::all() {
            let next = dir.next(&pos);
            if !walls.contains(&next) {
                queue.push_back((next, score + 1));
            }
        }
    }

    visited
}

/// Checks if we can reach every two points with [`max_cheat`] number then
/// check if the difference between their distance is bigger than [`min_diff`].
/// Each case of those is counted because we are starting with one valid way so we
/// don't have cases were the maze can't be solved.
fn solve_with_manhattan_dist(input: &str, max_cheat: usize, min_diff: usize) -> usize {
    let StartInfo {
        walls,
        start,
        end,
        rows_count: _,
        cols_count: _,
    } = parse(input);

    let pos_scores = positions_scores(&start, &end, &walls);
    let mut sum = 0;

    for items in pos_scores.iter().combinations(2) {
        let (p1, &score1) = items[0];
        let (p2, &score2) = items[1];
        let manhattan_dist = p1.row.abs_diff(p2.row) + p1.col.abs_diff(p2.col);
        if manhattan_dist <= max_cheat && score1.abs_diff(score2) >= manhattan_dist + min_diff {
            sum += 1;
        }
    }

    sum
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "20").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_solution() {
        let diffs = cheat_counts(INPUT);
        dbg!(diffs);

        println!("Test here is for debugging purpose only")
    }
}
