use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
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

    fn next(self, pos: Pos) -> Pos {
        match self {
            Dir::Up => Pos::new(pos.x.saturating_sub(1), pos.y),
            Dir::Right => Pos::new(pos.x, pos.y + 1),
            Dir::Down => Pos::new(pos.x + 1, pos.y),
            Dir::Left => Pos::new(pos.x, pos.y.saturating_sub(1)),
        }
    }
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Pos::new(x, y)
        })
        .collect()
}

fn calc_shortest(input: &str, take: usize, target: Pos) -> usize {
    let all_obsts = parse(input);
    let involved_obsts: HashSet<_> = all_obsts.into_iter().take(take).collect();
    let Pos { x: max_x, y: max_y } = target;

    let mut shortest = usize::MAX;

    let start = Pos::new(0, 0);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited: HashMap<Pos, usize> = HashMap::new();

    while let Some((current, score)) = queue.pop_front() {
        if current == target {
            shortest = shortest.min(score);
            continue;
        }

        match visited.entry(current) {
            Entry::Occupied(mut occupied_entry) => {
                if *occupied_entry.get() > score {
                    _ = occupied_entry.insert(score);
                } else {
                    continue;
                }
            }
            Entry::Vacant(vacant_entry) => _ = vacant_entry.insert(score),
        }

        for dir in Dir::all() {
            let next = dir.next(current);
            if next.x > max_x || next.y > max_y {
                continue;
            }

            if involved_obsts.contains(&next) {
                continue;
            }

            queue.push_back((next, score + 1));
        }
    }

    shortest
}

fn first_blocking(input: &str, skip: usize, target: Pos) -> Pos {
    let all_obsts = parse(input);
    let mut involved = HashSet::new();
    for (idx, obst) in all_obsts.into_iter().enumerate() {
        involved.insert(obst);
        if idx <= skip {
            continue;
        }
        if !check_way(&involved, target) {
            return obst;
        }
    }

    unreachable!()
}

fn check_way(involved_obsts: &HashSet<Pos>, target: Pos) -> bool {
    let Pos { x: max_x, y: max_y } = target;

    let start = Pos::new(0, 0);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited: HashSet<Pos> = HashSet::new();

    while let Some((current, score)) = queue.pop_front() {
        if current == target {
            return true;
        }

        if !visited.insert(current) {
            continue;
        }

        for dir in Dir::all() {
            let next = dir.next(current);
            if next.x > max_x || next.y > max_y {
                continue;
            }

            if involved_obsts.contains(&next) {
                continue;
            }

            queue.push_back((next, score + 1));
        }
    }

    false
}

fn part_1(input: &'static str) {
    let ans = calc_shortest(input, 1024, Pos::new(70, 70));
    println!("Part 1 answer is {ans}")
}

fn part_2(input: &'static str) {
    let ans = first_blocking(input, 1024, Pos::new(70, 70));
    println!("Part 2 answer is '{ans}'");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "18").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_solution() {
        let score = calc_shortest(INPUT, 12, Pos::new(6, 6));
        assert_eq!(score, 22);

        let block = first_blocking(INPUT, 12, Pos::new(6, 6));
        assert_eq!(block.to_string().as_str(), "6,1");
    }
}
