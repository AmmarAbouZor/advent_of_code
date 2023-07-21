use std::collections::VecDeque;

#[derive(Debug)]
struct Elf {
    id: usize,
    presents: usize,
}

impl Elf {
    fn new(id: usize, presents: usize) -> Self {
        Self { id, presents }
    }
}

fn part_1() {
    let mut elves = create_elves(3014603);
    // let mut elves = create_elves(5);
    while elves.len() > 1 {
        for pair in elves.chunks_mut(2) {
            if pair.len() == 2 {
                pair[0].presents += pair[1].presents;
                pair[1].presents = 0;
            }
        }

        let len = elves.len();
        if is_odd(len) {
            elves[len - 1].presents += elves[0].presents;
            elves[0].presents = 0;
        }

        elves.retain(|num| num.presents != 0);
    }

    println!("part_1: lucky elf is {} ", elves[0].id);
}

fn create_elves(size: usize) -> Vec<Elf> {
    (0..size).map(|i| Elf::new(i + 1, 1)).collect()
}

fn is_odd(num: usize) -> bool {
    num & 1 != 0
}

fn part_2() {
    let input = 3014603;
    let mut left: VecDeque<i32> = (1..input / 2 + 1).collect();
    let mut right: VecDeque<i32> = (input / 2 + 1..input + 1).rev().collect();

    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    let winner = left.front().unwrap_or_else(|| right.front().unwrap());

    println!("part_2: lucky elf is {winner}");
}

pub fn run() {
    part_1();
    part_2();
}
