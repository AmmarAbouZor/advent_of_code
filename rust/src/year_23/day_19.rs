use std::collections::{HashMap, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
struct MachineParts {
    scores: [usize; 4],
}

impl From<&str> for MachineParts {
    fn from(line: &str) -> Self {
        let line = line.trim_matches(['{', '}'].as_slice());

        let mut scores = [0; 4];

        line.split(',').enumerate().for_each(|(idx, part)| {
            let (_, num) = part.split_once('=').unwrap();
            scores[idx] = num.parse().unwrap();
        });

        Self { scores }
    }
}

#[derive(Debug, Clone)]
enum Inst<'a> {
    GoTo(&'a str),
    Accepted,
    Rejected,
}

impl<'a> From<&'a str> for Inst<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Inst::Accepted,
            "R" => Inst::Rejected,
            address => Inst::GoTo(address),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rule {
    GreaterThan(Category, usize),
    SmallerThan(Category, usize),
    None,
}

impl Rule {
    fn apply(&self, machine_parts: &MachineParts) -> bool {
        match self {
            Rule::GreaterThan(cat, val) => machine_parts.scores[*cat as usize] > *val,
            Rule::SmallerThan(cat, val) => machine_parts.scores[*cat as usize] < *val,
            Rule::None => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Condition<'a> {
    rule: Rule,
    inst: Inst<'a>,
}

impl<'a> Condition<'a> {
    fn new(rule: Rule, inst: Inst<'a>) -> Self {
        Self { rule, inst }
    }

    fn apply(&self, machine_parts: &MachineParts) -> Option<Inst<'a>> {
        if self.rule.apply(machine_parts) {
            Some(self.inst.clone())
        } else {
            None
        }
    }
}

fn parse_workflows(text: &str) -> HashMap<&str, Vec<Condition<'_>>> {
    text.lines()
        .map(|line| {
            let (key, mut conditions) = line.split_once('{').unwrap();
            conditions = conditions.trim_end_matches('}');
            let conditions = conditions
                .split(',')
                .map(|cond| {
                    if let Some((rule, inst)) = cond.split_once(':') {
                        let category = match rule.as_bytes()[0] {
                            b'x' => Category::X,
                            b'm' => Category::M,
                            b'a' => Category::A,
                            b's' => Category::S,
                            _ => unreachable!(),
                        };

                        let num = rule[2..].parse().unwrap();

                        let r = match rule.as_bytes()[1] {
                            b'>' => Rule::GreaterThan(category, num),
                            b'<' => Rule::SmallerThan(category, num),
                            _ => unreachable!(),
                        };

                        let inst = Inst::from(inst);
                        Condition::new(r, inst)
                    } else {
                        let inst = Inst::from(cond);
                        Condition::new(Rule::None, inst)
                    }
                })
                .collect();

            (key, conditions)
        })
        .collect()
}

fn get_accepted_sum(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows_map = parse_workflows(workflows);
    let parts: Vec<_> = parts.lines().map(MachineParts::from).collect();

    let mut accepted_sum = 0;

    for part in parts {
        let mut conditions = workflows_map.get("in").unwrap();
        loop {
            let res = conditions
                .iter()
                .find_map(|cond| cond.apply(&part))
                .unwrap();
            match res {
                Inst::GoTo(address) => conditions = workflows_map.get(address).unwrap(),
                Inst::Accepted => break accepted_sum += part.scores.iter().sum::<usize>(),
                Inst::Rejected => break,
            }
        }
    }

    accepted_sum
}

// Get combinations sum using ranges calculations
fn get_combinations(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows_map = parse_workflows(workflows);

    let start_ranges = vec![1..=4000; 4];

    let mut queue = VecDeque::new();
    queue.push_back((Inst::GoTo("in"), start_ranges));

    let mut accepted_ranges = Vec::new();

    while let Some((inst, mut ranges)) = queue.pop_front() {
        let inst_address = match inst {
            Inst::GoTo(address) => address,
            Inst::Accepted => {
                accepted_ranges.push(ranges);
                continue;
            }
            Inst::Rejected => continue,
        };

        let conditions = workflows_map.get(inst_address).unwrap();

        for cond in conditions.iter() {
            match cond.rule {
                Rule::None => {
                    queue.push_back((cond.inst.clone(), ranges));
                    break;
                }
                Rule::SmallerThan(cat, val) => {
                    let mut rng_clone = ranges.clone();
                    let cat_idx = cat as usize;

                    rng_clone[cat_idx] =
                        *rng_clone[cat_idx].start()..=(*rng_clone[cat_idx].end().min(&val) - 1);
                    queue.push_back((cond.inst.clone(), rng_clone));

                    ranges[cat_idx] = *ranges[cat_idx].start().max(&val)..=*ranges[cat_idx].end();
                }
                Rule::GreaterThan(cat, val) => {
                    let mut rng_clone = ranges.clone();
                    let cat_idx = cat as usize;

                    rng_clone[cat_idx] =
                        (*rng_clone[cat_idx].start().max(&val) + 1)..=*rng_clone[cat_idx].end();
                    queue.push_back((cond.inst.clone(), rng_clone));

                    ranges[cat_idx] = *ranges[cat_idx].start()..=*ranges[cat_idx].end().min(&val);
                }
            }
        }
    }

    accepted_ranges
        .into_iter()
        .map(|parts| parts.into_iter().map(|rng| rng.count()).product::<usize>())
        .sum()
}

fn part_1(input: &str) {
    let answer = get_accepted_sum(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = get_combinations(input);

    println!("Part 2 answer is {answer}")
}

pub fn run() {
    let input = read_text_from_file("23", "19");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_solution() {
        assert_eq!(get_accepted_sum(INPUT), 19114);
        assert_eq!(get_combinations(INPUT), 167409079868000);
    }
}
