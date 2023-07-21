use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
enum OperationKind {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    kind: OperationKind,
    val: Val,
}

#[derive(Debug)]
enum Val {
    Num(usize),
    Old,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
    insprected_count: usize,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);

        let items: VecDeque<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|num| num.trim_end_matches(',').parse().unwrap())
            .collect();

        let oper_infos: Vec<&str> = lines.next().unwrap().split_whitespace().skip(4).collect();

        let val = oper_infos[1].parse().map_or(Val::Old, Val::Num);

        let operation_kind = match oper_infos[0] {
            "+" => OperationKind::Add,
            _ => OperationKind::Multiply,
        };

        let operation = Operation {
            kind: operation_kind,
            val,
        };

        let divisor: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()?;

        let monkey_if_true: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()?;

        let monkey_if_false: usize = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()?;

        Ok(Monkey {
            items,
            operation,
            divisor,
            monkey_if_true,
            monkey_if_false,
            insprected_count: 0,
        })
    }
}

#[derive(Debug)]
struct InspectResult {
    item: usize,
    monkey_id: usize,
}

impl Monkey {
    fn inpsect<F>(&mut self, reduce_panic: F) -> Option<InspectResult>
    where
        F: Fn(usize) -> usize,
    {
        let item = self.items.pop_front()?;

        self.insprected_count += 1;

        let val = match self.operation.val {
            Val::Num(num) => num,
            Val::Old => item,
        };

        let mut worry_lvl = match self.operation.kind {
            OperationKind::Add => item + val,
            OperationKind::Multiply => item * val,
        };

        worry_lvl = reduce_panic(worry_lvl);

        let monkey_id = if worry_lvl % self.divisor == 0 {
            self.monkey_if_true
        } else {
            self.monkey_if_false
        };

        Some(InspectResult {
            item: worry_lvl,
            monkey_id,
        })
    }
}

fn calc_monkey_business(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| monkey.parse().unwrap())
        .collect();
    let reduce_panic = |num| num / 3;

    for _ in 0..20 {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];

            let mut results = Vec::new();

            while let Some(result) = monkey.inpsect(reduce_panic) {
                results.push(result);
            }

            for result in results {
                monkeys[result.monkey_id].items.push_back(result.item);
            }
        }
    }

    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.insprected_count).collect();

    scores.sort();

    scores.iter().rev().take(2).product()
}

fn calc_monkey_business_self_managed(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| monkey.parse().unwrap())
        .collect();

    let div_prod: usize = monkeys.iter().map(|m| m.divisor).product();
    let reduce_panic = |num| num % div_prod;

    for _ in 0..10000 {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];

            let mut results = Vec::new();

            while let Some(result) = monkey.inpsect(reduce_panic) {
                results.push(result);
            }

            for result in results {
                monkeys[result.monkey_id].items.push_back(result.item);
            }
        }
    }

    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.insprected_count).collect();

    scores.sort();

    scores.iter().rev().take(2).product()
}

fn part_1() {
    let input = read_text_from_file("22", "11");
    let score = calc_monkey_business(&input);

    println!("Part 1 answer is {score}");
}

fn part_2() {
    let input = read_text_from_file("22", "11");
    let score = calc_monkey_business_self_managed(&input);

    println!("Part 2 answer is {score}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_monkey_business(INPUT), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_monkey_business_self_managed(INPUT), 2713310158);
    }
}
