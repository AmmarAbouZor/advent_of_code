use std::collections::BTreeSet;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(isize),
    Jump(isize),
    NoOperation(isize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (name, num) = value.split_once(' ').unwrap();
        let num = num.parse().unwrap();
        match name {
            "acc" => Instruction::Acc(num),
            "jmp" => Instruction::Jump(num),
            "nop" => Instruction::NoOperation(num),
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
            Instruction::NoOperation(_) => 1,
        }
    }

    fn get_swap(&self) -> Option<Instruction> {
        match self {
            Instruction::Acc(_) => None,
            Instruction::Jump(num) => Some(Instruction::NoOperation(*num)),
            Instruction::NoOperation(num) => Some(Instruction::Jump(*num)),
        }
    }
}

fn get_value_by_infinite(input: &str) -> isize {
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

fn try_solve(
    mut curr_idx: isize,
    mut acc: isize,
    mut visited_set: BTreeSet<isize>,
    insts: &[Instruction],
) -> Option<isize> {
    while curr_idx < insts.len() as isize {
        if !visited_set.insert(curr_idx) {
            return None;
        }

        curr_idx += insts[curr_idx as usize].apply(&mut acc);
    }

    Some(acc)
}

fn get_value_after_correction(input: &str) -> isize {
    let insts: Vec<Instruction> = input.lines().map(Instruction::from).collect();

    let mut acc = 0;
    let mut visited_set = BTreeSet::new();

    let mut current_index = 0;

    loop {
        if !visited_set.insert(current_index) {
            unreachable!("No duplicate are allowed in this loop");
        }

        if let Some(swap) = insts[current_index as usize].get_swap() {
            let mut acc_clone = acc;
            let mut idx_clone = current_index;
            idx_clone += swap.apply(&mut acc_clone);
            if let Some(acc) = try_solve(idx_clone, acc_clone, visited_set.clone(), &insts) {
                return acc;
            }
        }

        current_index += insts[current_index as usize].apply(&mut acc);
    }
}

fn part_1() {
    let input = read_text_from_file("20", "08");
    let answer = get_value_by_infinite(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "08");
    let answer = get_value_after_correction(&input);

    println!("Part 2 answer is {answer}");
}

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
        assert_eq!(get_value_by_infinite(INPUT), 5);
        assert_eq!(get_value_after_correction(INPUT), 8);
    }
}
