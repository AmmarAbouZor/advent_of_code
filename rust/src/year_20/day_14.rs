use std::collections::BTreeMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct Operation {
    address: usize,
    num: usize,
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some(stripped) = value.strip_prefix("mem[") else {
            return Err("Line doesn't start with mem[");
        };

        let (address, num) = stripped.split_once("] = ").unwrap();
        let address = address.parse().unwrap();
        let num = num.parse().unwrap();

        Ok(Operation { address, num })
    }
}

#[derive(Debug)]
struct Program {
    and_mask: usize,
    or_mask: usize,
    operations: Vec<Operation>,
}

impl Program {
    fn new(mask: &str) -> Self {
        let and_mask = mask.replace('X', "1");
        let and_mask = usize::from_str_radix(and_mask.as_str(), 2).unwrap();
        let or_mask = mask.replace('X', "0");
        let or_mask = usize::from_str_radix(or_mask.as_str(), 2).unwrap();

        Program {
            and_mask,
            or_mask,
            operations: Vec::new(),
        }
    }

    fn apply(&self, memory_map: &mut BTreeMap<usize, usize>) {
        for operation in self.operations.iter() {
            let and_value = operation.num & self.and_mask;
            let full_value = and_value | self.or_mask;

            memory_map
                .entry(operation.address)
                .and_modify(|val| *val = full_value)
                .or_insert(full_value);
        }
    }
}

fn parse_input(input: &str) -> Vec<Program> {
    let mut programs = Vec::new();
    for line in input.lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            let program = Program::new(mask);
            programs.push(program);
        } else {
            let last_program = programs.last_mut().unwrap();
            let operation = Operation::try_from(line).unwrap();
            last_program.operations.push(operation);
        }
    }

    programs
}

fn get_sum_memory(input: &str) -> usize {
    let programs = parse_input(input);
    let mut memory_map = BTreeMap::new();

    for program in programs.iter() {
        program.apply(&mut memory_map);
    }

    memory_map.values().sum()
}

fn part_1(input: &str) {
    let answer_1 = get_sum_memory(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("20", "14");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part_1() {
        assert_eq!(get_sum_memory(INPUT), 165);
    }
}

