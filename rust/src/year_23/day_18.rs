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

        Self { dir, count }
    }
}

impl Inst {
    fn from_hex(line: &str) -> Self {
        let (_, mut hex) = line.split_once("(#").unwrap();
        hex = hex.trim_end_matches(')');
        let dir = match isize::from_str_radix(&hex[5..], 16).unwrap() {
            0 => Dir::East,
            1 => Dir::South,
            2 => Dir::West,
            3 => Dir::North,
            invalid => unreachable!("{invalid}"),
        };

        let count = isize::from_str_radix(&hex[..5], 16).unwrap();

        Self { dir, count }
    }
}

fn get_vertices<F>(input: &str, parse_func: F) -> Vec<Point>
where
    F: Fn(&str) -> Inst,
{
    input
        .lines()
        .map(parse_func)
        .fold(vec![Point::new(0, 0)], |mut vertices, inst| {
            let next_dir = match inst.dir {
                Dir::North => (0, 1),
                Dir::East => (1, 0),
                Dir::South => (0, -1),
                Dir::West => (-1, 0),
            };

            let mut vertex = *vertices.last().unwrap();
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

    ((area.unsigned_abs() / 2) - (perimeter / 2) + 1) + perimeter
}

fn calc_cubes_count(input: &str) -> usize {
    let vertices = get_vertices(input, |line| Inst::from(line));
    calc_filled(&vertices)
}

fn calc_from_hex(input: &str) -> usize {
    let vertices = get_vertices(input, Inst::from_hex);
    calc_filled(&vertices)
}

fn part_1(input: &str) {
    let answer = calc_cubes_count(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_from_hex(input);

    println!("Part 2 answer is {answer}");
}

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
        assert_eq!(calc_from_hex(INPUT), 952408144115);
    }
}

