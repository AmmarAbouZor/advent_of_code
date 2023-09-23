use std::{collections::HashMap, isize};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(char),
    Add(char, Value),
    Multiply(char, Value),
    Divide(char, Value),
    Mod(char, Value),
    Equal(char, Value),
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Digit(isize),
    Variable(char),
}

impl Value {
    fn get_val(&self, varaiables: &HashMap<char, isize>) -> isize {
        match self {
            Value::Digit(digit) => *digit,
            Value::Variable(ch) => *varaiables.get(ch).unwrap_or(&0),
        }
    }
}

impl From<char> for Value {
    fn from(ch: char) -> Self {
        match ch.to_digit(10) {
            Some(digit) => Value::Digit(digit as isize),
            None => Value::Variable(ch),
        }
    }
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        use Instruction as I;
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let char_1 = parts.next().unwrap().chars().next().unwrap();
        let char_2 = parts.next().and_then(|text| text.chars().next());
        match name {
            "inp" => I::Input(char_1),
            "add" => I::Add(char_1, char_2.unwrap().into()),
            "mul" => I::Multiply(char_1, char_2.unwrap().into()),
            "div" => I::Divide(char_1, char_2.unwrap().into()),
            "mod" => I::Mod(char_1, char_2.unwrap().into()),
            "eql" => I::Equal(char_1, char_2.unwrap().into()),
            invalid => unreachable!("invalid input {invalid}"),
        }
    }
}

impl Instruction {
    fn apply(
        &self,
        variables: &mut HashMap<char, isize>,
        input: &mut Vec<isize>,
    ) -> Result<(), &str> {
        let get_two_value = |a, b: &Value| {
            let a_val = variables.get(a).unwrap_or(&0);
            let b_val = b.get_val(variables);
            (*a_val, b_val)
        };
        match self {
            Instruction::Input(a) => {
                let val = input.pop().unwrap();
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
            Instruction::Add(a, b) => {
                let (a_val, b_val) = get_two_value(a, b);
                let val = a_val + b_val;
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
            Instruction::Multiply(a, b) => {
                let (a_val, b_val) = get_two_value(a, b);
                let val = a_val * b_val;
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
            Instruction::Divide(a, b) => {
                let (a_val, b_val) = get_two_value(a, b);
                if b_val == 0 {
                    return Err("Divide by zero");
                }
                let val = a_val / b_val;
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
            Instruction::Mod(a, b) => {
                let (a_val, b_val) = get_two_value(a, b);
                if a_val.is_negative() || b_val.is_negative() {
                    return Err("Mod negative value");
                }
                let val = a_val % b_val;
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
            Instruction::Equal(a, b) => {
                let (a_val, b_val) = get_two_value(a, b);
                let val = if a_val == b_val { 1 } else { 0 };
                variables.entry(*a).and_modify(|v| *v = val).or_insert(val);
                Ok(())
            }
        }
    }
}

fn num_to_valid_rev_digits(mut num: isize) -> Option<Vec<isize>> {
    let mut digits = Vec::with_capacity(14);
    while num.is_positive() {
        let digit = num % 10;
        if digit == 0 {
            return None;
        }
        digits.push(digit);
        num /= 10;
    }

    assert_eq!(digits.len(), 14);

    Some(digits)
}

fn find_max_valid(input: &str) -> isize {
    let insts: Vec<_> = input.lines().map(Instruction::from).collect();
    let nums = (111_111_111_111_11..=999_999_999_999_99)
        .rev()
        .flat_map(|num| num_to_valid_rev_digits(num).map(|digits| (num, digits)));
    let mut counter = 0;
    for (num, mut digits) in nums {
        counter += 1;

        if counter % 500 == 0 {
            println!("{num}");
        }

        let mut variables = HashMap::new();
        let has_errors = insts
            .iter()
            .any(|ins| ins.apply(&mut variables, &mut digits).is_err());
        if !has_errors && *variables.get(&'z').unwrap() == 0 {
            return num;
        }
    }

    unreachable!()
}

fn part_1() {
    let input = read_text_from_file("21", "24");
    let answer = find_max_valid(input.as_str());

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

    #[test]
    fn test_num_to_digit() {
        let input = 12345678912345;
        assert_eq!(
            num_to_valid_rev_digits(input),
            Some(vec![5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1])
        );

        // Zero is invalid input
        assert_eq!(num_to_valid_rev_digits(10234567891234), None);
    }
}
