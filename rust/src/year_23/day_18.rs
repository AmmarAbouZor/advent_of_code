use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Inst {
    dir: Dir,
    count: isize,
    color: u64,
}

impl From<&str> for Inst {
    fn from(line: &str) -> Self {
        let mut parts = line.split_ascii_whitespace();
        let dir = match parts.next().unwrap() {
            "U" => Dir::North,
            "R" => Dir::East,
            "D" => Dir::South,
            "L" => Dir::West,
            invalid => unreachable!("invalid input '{invalid}'"),
        };

        let count = parts.next().and_then(|num| num.parse().ok()).unwrap();

        let mut color = parts.next().unwrap();

        color = color.trim_start_matches("(#");
        color = color.trim_end_matches(')');
        let color = u64::from_str_radix(color, 16).unwrap();

        Self { dir, count, color }
    }
}

fn get_vertices(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(Inst::from)
        .fold(vec![Point::new(0, 0)], |mut vertices, inst| {
            let next_dir = match inst.dir {
                Dir::North => (0, 1),
                Dir::East => (1, 0),
                Dir::South => (0, -1),
                Dir::West => (-1, 0),
            };

            let mut vertex = vertices.last().unwrap().clone();
            vertex.x += next_dir.0 * inst.count;
            vertex.y += next_dir.1 * inst.count;

            vertices.push(vertex);

            vertices
        })
}

// Calculation is done using Shoelace Algorithm
fn calc_filled(vertices: &[Point]) -> usize {
    let mut area = 0;
    let mut perimeter = 0;
    for window in vertices.windows(2) {
        let first = window[0];
        let second = window[1];
        area += (first.x * second.y) - (first.y * second.x);
        let perimeter_diff = first.x.abs_diff(second.x) + first.y.abs_diff(second.y);
        perimeter += perimeter_diff;
    }

    ((area.abs() as usize / 2) - (perimeter / 2) + 1) + perimeter
}

fn calc_cubes_count(input: &str) -> usize {
    let vertcies = get_vertices(input);
    calc_filled(&vertcies)
}

fn part_1(input: &str) {
    let answer = calc_cubes_count(input);

    println!("Part 1 asnwer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "18");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_solution() {
        assert_eq!(calc_cubes_count(INPUT), 62);
    }
}

