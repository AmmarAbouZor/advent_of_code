use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
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
    fn calc_surface_area(&self, other_cubes: &[Cube]) -> usize {
        let mut sides = 6;
        let x_dir_cubes: Vec<&Cube> = other_cubes
            .iter()
            .filter(|cube| cube.y == self.y && cube.z == self.z)
            .collect();
        if x_dir_cubes.iter().any(|cube| cube.x - self.x == 1) {
            sides -= 1;
        }
        if x_dir_cubes.iter().any(|cube| cube.x - self.x == -1) {
            sides -= 1;
        }

        let y_dir_cubes: Vec<&Cube> = other_cubes
            .iter()
            .filter(|cube| cube.x == self.x && cube.z == self.z)
            .collect();
        if y_dir_cubes.iter().any(|cube| cube.y - self.y == 1) {
            sides -= 1;
        }
        if y_dir_cubes.iter().any(|cube| cube.y - self.y == -1) {
            sides -= 1;
        }

        let z_dir_cubes: Vec<&Cube> = other_cubes
            .iter()
            .filter(|cube| cube.y == self.y && cube.x == self.x)
            .collect();
        if z_dir_cubes.iter().any(|cube| cube.z - self.z == 1) {
            sides -= 1;
        }
        if z_dir_cubes.iter().any(|cube| cube.z - self.z == -1) {
            sides -= 1;
        }

        sides
    }
}

fn calc_surface_area(input: &str) -> usize {
    let cubes: Vec<Cube> = input.lines().flat_map(Cube::from_str).collect();

    cubes.iter().map(|c| c.calc_surface_area(&cubes)).sum()
}

fn calc_exterior_surface(input: &str) -> usize {
    let cubes: Vec<Cube> = input.lines().flat_map(Cube::from_str).collect();

    let all_surface_area: usize = cubes.iter().map(|c| c.calc_surface_area(&cubes)).sum();

    let mut x_gaps: BTreeSet<isize> = BTreeSet::new();

    for cube in cubes.iter() {
        if cubes
            .iter()
            .filter(|c| c.y == cube.y && c.z == cube.z && c.x > cube.x)
            .all(|c| c.x - cube.x > 0)
        {
            x_gaps.insert(cube.x + 1);
        }
    }

    dbg!(x_gaps.len());

    let mut y_gaps: BTreeSet<(isize, isize)> = BTreeSet::new();

    for cube in x_gaps.iter().flat_map(|x| cubes.iter().find(|c| c.x == *x)) {
        if cubes
            .iter()
            .filter(|c| c.x == cube.x && c.z == cube.z && c.y > cube.y)
            .all(|c| c.y - cube.y > 0)
        {
            y_gaps.insert((cube.x, cube.y + 1));
        }
    }

    dbg!(y_gaps.len());

    let mut z_gaps: BTreeSet<(isize, isize, isize)> = BTreeSet::new();

    for cube in y_gaps
        .iter()
        .flat_map(|(_x1, y2)| cubes.iter().find(|c| c.y == *y2))
    {
        if cubes
            .iter()
            .filter(|c| c.x == cube.x && c.y == cube.y && c.z > cube.z)
            .all(|c| c.z - cube.z > 0)
        {
            z_gaps.insert((cube.x, cube.y, cube.z + 1));
        }
    }

    dbg!(z_gaps.len());

    all_surface_area - (z_gaps.len() * 6)
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
    #[ignore]
    fn test_part_1() {
        assert_eq!(calc_surface_area(INPUT), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_exterior_surface(INPUT), 58);
    }
}
