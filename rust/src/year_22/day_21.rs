use std::collections::{HashMap, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Operation {
    Sum,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Calculation {
    left: String,
    right: String,
    operation: Operation,
}
#[derive(Debug)]
enum Job {
    Yell(isize),
    Calc(Calculation),
}
#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl From<&str> for Monkey {
    fn from(line: &str) -> Self {
        let (name, job) = line.split_once(": ").unwrap();
        let name = name.to_owned();

        let job = if let Ok(num) = job.parse::<isize>() {
            Job::Yell(num)
        } else {
            let mut parts = job.split_whitespace();

            let left = parts.next().unwrap().to_owned();
            let operation = match parts.next().unwrap() {
                "+" => Operation::Sum,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => unreachable!("invalid input"),
            };
            let right = parts.next().unwrap().to_owned();

            Job::Calc(Calculation {
                left,
                operation,
                right,
            })
        };

        Monkey { name, job }
    }
}

impl Monkey {
    fn try_solve(&self, map: &HashMap<String, isize>) -> Option<isize> {
        match &self.job {
            Job::Yell(num) => Some(*num),
            Job::Calc(calc) => {
                let left_val = map.get(&calc.left)?;
                let right_val = map.get(&calc.right)?;

                let val = match calc.operation {
                    Operation::Sum => left_val + right_val,
                    Operation::Subtract => left_val - right_val,
                    Operation::Multiply => left_val * right_val,
                    Operation::Divide => left_val / right_val,
                };

                Some(val)
            }
        }
    }

    fn solve_back(&self, hashes: &mut HashMap<String, isize>) {
        let val = *hashes.get(&self.name).unwrap();
        if let Job::Calc(calc) = &self.job {
            match (hashes.get(&calc.left), hashes.get(&calc.right)) {
                (Some(left), None) => {
                    let right = match calc.operation {
                        Operation::Sum => val - left,
                        Operation::Subtract => left - val,
                        Operation::Multiply => val / left,
                        Operation::Divide => left / val,
                    };

                    hashes.insert(calc.right.to_owned(), right);
                }
                (None, Some(right)) => {
                    let left = match calc.operation {
                        Operation::Sum => val - right,
                        Operation::Subtract => right + val,
                        Operation::Multiply => val / right,
                        Operation::Divide => right * val,
                    };

                    hashes.insert(calc.left.to_owned(), left);
                }
                _ => (),
            }
        } else {
            unreachable!();
        }
    }
}

fn get_root_val(input: &str) -> isize {
    let mut monkeys: VecDeque<Monkey> = input.lines().map(Monkey::from).collect();

    let mut hashes = HashMap::new();

    while let Some(monkey) = monkeys.pop_back() {
        if let Some(val) = monkey.try_solve(&hashes) {
            if monkey.name == "root" {
                return val;
            };
            hashes.insert(monkey.name.to_owned(), val);
        } else {
            monkeys.push_front(monkey);
        }
    }

    unreachable!();
}

fn get_humn_val(input: &str) -> isize {
    let mut monkeys: VecDeque<Monkey> = input.lines().map(Monkey::from).collect();

    // reverse humn from my input
    // rqmm: humn - jcmg
    let humn = monkeys.iter_mut().find(|m| m.name == "humn").unwrap();
    humn.job = Job::Calc(Calculation {
        left: "rqmm".to_owned(),
        right: "jcmg".to_owned(),
        operation: Operation::Sum,
    });

    let mut hashes = HashMap::new();

    while let Some(monkey) = monkeys.pop_back() {
        if monkey.name == "root" {
            if let Job::Calc(calc) = &monkey.job {
                match (hashes.get(&calc.left), hashes.get(&calc.right)) {
                    (Some(val), None) => hashes.insert(calc.right.to_owned(), *val),
                    (None, Some(val)) => hashes.insert(calc.left.to_owned(), *val),
                    _ => None,
                };
            } else {
                unreachable!();
            }
        }

        if let Some(val) = monkey.try_solve(&hashes) {
            if monkey.name == "humn" {
                return val;
            };
            hashes.insert(monkey.name.to_owned(), val);
        } else {
            if hashes.get(&monkey.name).is_some() {
                monkey.solve_back(&mut hashes);
            }
            monkeys.push_front(monkey);
        }
    }

    unreachable!();
}

fn part_1() {
    let input = read_text_from_file("22", "21");

    let answer = get_root_val(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "21");

    let answer = get_humn_val(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part_1() {
        assert_eq!(get_root_val(INPUT), 152);
    }
}
