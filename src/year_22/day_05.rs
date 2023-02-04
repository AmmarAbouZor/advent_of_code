use std::collections::{BTreeMap, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Move {
    from: char,
    to: char,
    amount: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();

        let amount = parts[1].parse().unwrap();

        let from = parts[3].chars().next().unwrap();
        let to = parts[5].chars().next().unwrap();

        Move { from, to, amount }
    }
}

fn parse_stacks(text: &str) -> BTreeMap<char, VecDeque<char>> {
    let mut lines: Vec<&str> = text.lines().collect();

    let columns_map: BTreeMap<usize, char> = lines
        .pop()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_index, stack)| !stack.is_whitespace())
        .collect();

    let mut stacks = BTreeMap::new();

    for line in lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for col in columns_map.iter() {
            let crat = chars[*col.0];
            if !crat.is_whitespace() {
                let stack = stacks.entry(*col.1).or_insert(VecDeque::new());
                stack.push_front(crat);
            }
        }
    }

    stacks
}

fn get_up_crates(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);

    let moves: Vec<Move> = moves.lines().map(Move::from).collect();

    for mv in moves {
        for _ in 0..mv.amount {
            let from_stack = stacks.get_mut(&mv.from).unwrap();
            let crat = from_stack.pop_front().unwrap();

            let to_stack = stacks.get_mut(&mv.to).unwrap();
            to_stack.push_front(crat);
        }
    }

    stacks
        .values()
        .map(|stack| stack.front().unwrap().to_owned())
        .collect()
}

fn get_up_crates_move_together(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);

    let moves: Vec<Move> = moves.lines().map(Move::from).collect();

    for mv in moves {
        let from_stack = stacks.get_mut(&mv.from).unwrap();
        let mv_crates: Vec<char> = (0..mv.amount)
            .map(|_| from_stack.pop_front().unwrap())
            .collect();

        let to_stack = stacks.get_mut(&mv.to).unwrap();

        mv_crates
            .iter()
            .rev()
            .for_each(|ch| to_stack.push_front(*ch));
    }

    stacks
        .values()
        .map(|stack| stack.front().unwrap().to_owned())
        .collect()
}

fn part_1() {
    let input = read_text_from_file("22", "05");

    let answer = get_up_crates(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "05");

    let answer = get_up_crates_move_together(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}
