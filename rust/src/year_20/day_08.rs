use std::collections::BTreeSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(isize),
    Jump(isize),
    NoOperation,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (name, num) = value.split_once(' ').unwrap();
        match name {
            "acc" => Instruction::Acc(num.parse().unwrap()),
            "jmp" => Instruction::Jump(num.parse().unwrap()),
            "nop" => Instruction::NoOperation,
            invalid => unreachable!("Invalid input: '{invalid}'"),
        }
    }
}

impl Instruction {
    fn apply(&self, acc: &mut isize) -> isize {
        match self {
            Instruction::Acc(value) => {
                *acc += *value;
                1
            }
            Instruction::Jump(offset) => *offset,
            Instruction::NoOperation => 1,
        }
    }
}

fn get_value_by_infinit(input: &str) -> isize {
    let insts: Vec<Instruction> = input.lines().map(Instruction::from).collect();

    let mut acc = 0;
    let mut visited_set = BTreeSet::new();

    let mut current_index = 0;

    loop {
        if !visited_set.insert(current_index) {
            return acc;
        }

        current_index += insts[current_index as usize].apply(&mut acc);
    }
}

fn part_1() {
    let input = read_text_from_file("20", "08");
    let answer = get_value_by_infinit(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_solution() {
        assert_eq!(get_value_by_infinit(INPUT), 5)
    }
}

