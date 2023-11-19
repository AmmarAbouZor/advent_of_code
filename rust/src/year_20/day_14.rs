use std::collections::BTreeMap;
use std::str;

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

impl Operation {
    fn apply_address(&self, mask: &str, memory_map: &mut BTreeMap<usize, usize>) {
        let mask = mask.as_bytes();
        let address_binary_text = format!("{:b}", self.address);
        let address_binary_text = address_binary_text.as_bytes();

        let result_binary_text: Vec<_> = address_binary_text
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, char)| match mask.iter().rev().nth(idx).unwrap() {
                b'0' => *char,
                b'1' => b'1',
                b'X' => b'X',
                invalide => unreachable!("Value is invalide {invalide}"),
            })
            .rev()
            .collect();

        // Debug code
        result_binary_text
            .iter()
            .for_each(|ch| print!("{}", *ch as char));
        println!();
        // End debug code

        let mut combinations = Vec::new();
        Self::get_all_combinations(result_binary_text, 0, &mut combinations);

        combinations
            .iter_mut()
            .map(|comb| str::from_utf8(comb).unwrap())
            .map(|comb| usize::from_str_radix(comb, 2).unwrap())
            .for_each(|address| {
                _ = memory_map
                    .entry(address)
                    .and_modify(|val| *val = self.num)
                    .or_insert(self.num)
            });
    }

    fn get_all_combinations(mut input: Vec<u8>, idx: usize, combinations: &mut Vec<Vec<u8>>) {
        if idx == input.len() {
            combinations.push(input);
            return;
        }

        match input[idx] {
            b'1' | b'0' => Self::get_all_combinations(input, idx + 1, combinations),
            b'X' => {
                let mut clone = input.clone();
                clone[idx] = b'0';
                Self::get_all_combinations(clone, idx + 1, combinations);
                input[idx] = b'1';
                Self::get_all_combinations(input, idx + 1, combinations);
            }
            invalid => unreachable!("Invalid input {invalid}"),
        }
    }
}

#[derive(Debug)]
struct Program<'a> {
    mask: &'a str,
    and_mask: usize,
    or_mask: usize,
    operations: Vec<Operation>,
}

impl Program<'_> {
    fn new(mask: &str) -> Program<'_> {
        let and_mask = mask.replace('X', "1");
        let and_mask = usize::from_str_radix(and_mask.as_str(), 2).unwrap();
        let or_mask = mask.replace('X', "0");
        let or_mask = usize::from_str_radix(or_mask.as_str(), 2).unwrap();

        Program {
            mask,
            and_mask,
            or_mask,
            operations: Vec::new(),
        }
    }

    fn apply_mask_value(&self, memory_map: &mut BTreeMap<usize, usize>) {
        for operation in self.operations.iter() {
            let and_value = operation.num & self.and_mask;
            let full_value = and_value | self.or_mask;

            memory_map
                .entry(operation.address)
                .and_modify(|val| *val = full_value)
                .or_insert(full_value);
        }
    }

    fn apply_mask_address(&self, memory_map: &mut BTreeMap<usize, usize>) {
        self.operations
            .iter()
            .for_each(|op| op.apply_address(self.mask, memory_map));
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

fn get_sum_memory_values(input: &str) -> usize {
    let programs = parse_input(input);
    let mut memory_map = BTreeMap::new();

    for program in programs.iter() {
        program.apply_mask_value(&mut memory_map);
    }

    memory_map.values().sum()
}

fn get_sum_memory_address(input: &str) -> usize {
    let programs = parse_input(input);
    let mut memory_map = BTreeMap::new();

    for program in programs.iter() {
        program.apply_mask_address(&mut memory_map);
    }

    memory_map.values().sum()
}

fn part_1(input: &str) {
    let answer_1 = get_sum_memory_values(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {
    let answer_2 = get_sum_memory_address(input);

    println!("Part 2 answer is {answer_2}");
}

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

    const INPUT_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part_1() {
        assert_eq!(get_sum_memory_values(INPUT), 165);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_sum_memory_address(INPUT_2), 208);
    }
}
