use std::collections::{HashMap, HashSet, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum Operaion {
    Sum,
    Subtract,
    Multiply,
    Divied,
}

#[derive(Debug)]
struct Calculation {
    left: String,
    right: String,
    operation: Operaion,
}
#[derive(Debug)]
enum Job {
    yell(isize),
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
            Job::yell(num)
        } else {
            let mut parts = job.split_whitespace();

            let left = parts.next().unwrap().to_owned();
            let operation = match parts.next().unwrap() {
                "+" => Operaion::Sum,
                "-" => Operaion::Subtract,
                "*" => Operaion::Multiply,
                "/" => Operaion::Divied,
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
            Job::yell(num) => Some(*num),
            Job::Calc(calc) => {
                let left_val = map.get(&calc.left)?;
                let right_val = map.get(&calc.right)?;

                let val = match calc.operation {
                    Operaion::Sum => left_val + right_val,
                    Operaion::Subtract => left_val - right_val,
                    Operaion::Multiply => left_val * right_val,
                    Operaion::Divied => left_val / right_val,
                };

                Some(val)
            }
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

fn part_1() {
    let input = read_text_from_file("22", "21");

    let answer = get_root_val(&input);

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
