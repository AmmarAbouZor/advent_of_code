use std::{
    collections::{BTreeSet, BinaryHeap},
    num::ParseIntError,
    str::FromStr,
};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Cube {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;
        let z = parts.next().unwrap().parse()?;

        Ok(Cube { x, y, z })
    }
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn get_possible_neighbors(&self) -> [Cube; 6] {
        [
            Cube {
                x: self.x + 1,
                ..*self
            },
            Cube {
                x: self.x - 1,
                ..*self
            },
            Cube {
                y: self.y + 1,
                ..*self
            },
            Cube {
                y: self.y - 1,
                ..*self
            },
            Cube {
                z: self.z + 1,
                ..*self
            },
            Cube {
                z: self.z - 1,
                ..*self
            },
        ]
    }
}

fn calc_surface_area(input: &str) -> usize {
    let cubes: BTreeSet<Cube> = input.lines().flat_map(Cube::from_str).collect();

    cubes
        .iter()
        .copied()
        .flat_map(|c| c.get_possible_neighbors())
        .filter(|c| !cubes.contains(c))
        .count()
}

fn calc_exterior_surface(input: &str) -> usize {
    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) = (
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
    );
    let cubes: BTreeSet<Cube> = input
        .lines()
        .flat_map(Cube::from_str)
        .inspect(|&cube| {
            min_x = min_x.min(cube.x);
            max_x = max_x.max(cube.x);
            min_y = min_y.min(cube.y);
            max_y = max_y.max(cube.y);
            min_z = min_z.min(cube.z);
            max_z = max_z.max(cube.z);
        })
        .collect();

    let mut exterior_heap = BinaryHeap::from([Cube::new(min_x - 1, min_y - 1, min_z - 1)]);
    let mut exterior_neighbors = BTreeSet::from_iter(exterior_heap.iter().copied());

    let x_range = min_x - 1..=max_x + 1;
    let y_range = min_y - 1..=max_y + 1;
    let z_range = min_z - 1..=max_z + 1;

    while let Some(cube) = exterior_heap.pop() {
        for neighbor in cube.get_possible_neighbors() {
            if x_range.contains(&neighbor.x)
                && y_range.contains(&neighbor.y)
                && z_range.contains(&neighbor.z)
                && !cubes.contains(&neighbor)
            {
                let is_new = exterior_neighbors.insert(neighbor);
                if is_new {
                    exterior_heap.push(neighbor);
                }
            }
        }
    }

    cubes
        .iter()
        .copied()
        .flat_map(|c| c.get_possible_neighbors())
        .filter(|c| exterior_neighbors.contains(c))
        .count()
}

fn part_1() {
    let input = read_text_from_file("22", "18");

    let answer = calc_surface_area(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "18");

    let answer = calc_exterior_surface(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_surface_area(INPUT), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_exterior_surface(INPUT), 58);
    }
}
