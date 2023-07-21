use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::utls::read_text_from_file;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_next_moves(
        &self,
        valid_pos: &HashSet<Point>,
        visited_pos: &HashSet<Point>,
    ) -> Vec<Point> {
        let mut moves = Vec::new();
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let point = Point::new(self.x + dx, self.y + dy);
            if !visited_pos.contains(&point) && valid_pos.contains(&point) {
                moves.push(point);
            }
        }

        moves
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn get_valid_points(input: &str) -> (HashMap<i32, Point>, HashSet<Point>) {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                // ignore walls
                '#' => {}
                '.' => {
                    set.insert(Point::new(x as i32, y as i32));
                }
                num => {
                    set.insert(Point::new(x as i32, y as i32));
                    let num = num.to_digit(10).unwrap() as i32;
                    map.insert(num, Point::new(x as i32, y as i32));
                }
            }
        }
    }

    (map, set)
}

fn calc_shortest_dist(from: Point, target: Point, valid_points: &HashSet<Point>) -> i32 {
    let mut states = VecDeque::new();
    states.push_front((0, from));
    let mut visited = HashSet::new();

    let mut distances = Vec::new();

    while !states.is_empty() {
        let (distance, pos) = states.pop_back().unwrap();
        if pos == target {
            distances.push(distance);
            continue;
        }

        for next_pos in pos.get_next_moves(valid_points, &visited) {
            states.push_front((distance + 1, next_pos));
            visited.insert(next_pos);
        }
    }

    *distances.iter().min().unwrap()
}

fn calc_all_distances(
    num_pos: &HashMap<i32, Point>,
    valid_pos: &HashSet<Point>,
) -> HashMap<(i32, i32), i32> {
    let mut distances = HashMap::new();
    for i in 0..num_pos.len() as i32 {
        for j in 0..num_pos.len() as i32 {
            if i != j && !distances.contains_key(&(i, j)) {
                let distance = calc_shortest_dist(num_pos[&i], num_pos[&j], valid_pos);
                distances.insert((i, j), distance);
                distances.insert((j, i), distance);
            }
        }
    }

    distances
}

fn calc_shortest_path(input: &str) -> i32 {
    let (map, valid_pos) = get_valid_points(input);

    dbg!(map.len());

    let distance = calc_all_distances(&map, &valid_pos);

    (1..map.len() as i32)
        .permutations(map.len() - 1)
        .map(|mut arr| {
            arr.insert(0, 0);
            let mut sum = 0;
            for i in 0..arr.len() - 1 {
                sum += distance[&(arr[i], arr[i + 1])];
            }
            sum
        })
        .min()
        .unwrap()
}

fn calc_shortest_path_and_pack(input: &str) -> i32 {
    let (map, valid_pos) = get_valid_points(input);

    let distance = calc_all_distances(&map, &valid_pos);

    (1..map.len() as i32)
        .permutations(map.len() - 1)
        .map(|mut arr| {
            arr.insert(0, 0);
            arr.push(0);
            let mut sum = 0;
            for i in 0..arr.len() - 1 {
                sum += distance[&(arr[i], arr[i + 1])];
            }
            sum
        })
        .min()
        .unwrap()
}

fn part_1() {
    let input = read_text_from_file("16", "24");
    let short_path = calc_shortest_path(&input);

    println!("part_1: fewest steps are {short_path}");
}

fn part_2() {
    let input = read_text_from_file("16", "24");
    let short_path = calc_shortest_path_and_pack(&input);

    println!("part_2: fewest steps are {short_path}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let input = r"###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

        assert_eq!(calc_shortest_path(input), 14);
    }
}
