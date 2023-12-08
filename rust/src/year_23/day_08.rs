use std::collections::HashMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Dir::Left,
            'R' => Dir::Right,
            invalid => unreachable!("Invalid input: '{invalid}'"),
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(line: &'a str) -> Self {
        let (name, mut rest) = line.split_once(" = ").unwrap();
        rest = rest.trim_start_matches("(");
        rest = rest.trim_end_matches(")");
        let (left, right) = rest.split_once(", ").unwrap();

        Self { name, left, right }
    }
}

impl<'a> Node<'a> {
    fn apply_dir(&self, dir: &Dir) -> &'a str {
        match dir {
            Dir::Left => self.left,
            Dir::Right => self.right,
        }
    }
}

fn parse_input<'a>(input: &'a str) -> (Vec<Dir>, HashMap<&'a str, Node<'a>>) {
    let (dirs, nodes) = input.split_once("\n\n").unwrap();
    let dirs = dirs.chars().map(Dir::from).collect();
    let nodes = nodes
        .lines()
        .map(Node::from)
        .map(|node| (node.name, node))
        .collect();

    (dirs, nodes)
}

fn calc_steps(input: &str) -> usize {
    let (dirs, nodes_map) = parse_input(input);
    let mut dirs = dirs.iter().cycle();
    let mut node = "AAA";

    let mut count = 0;
    while node != "ZZZ" {
        let current_node = nodes_map.get(node).unwrap();
        let dir = dirs.next().unwrap();
        node = current_node.apply_dir(dir);
        count += 1;
    }

    count
}

fn part_1(input: &str) {
    let answer = calc_steps(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "08");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_solution() {
        assert_eq!(calc_steps(INPUT_1), 2);
        assert_eq!(calc_steps(INPUT_2), 6);
    }
}

