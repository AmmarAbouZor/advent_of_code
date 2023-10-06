use std::collections::{HashMap, HashSet};

use crate::utls::read_text_from_file;

const TARGET_NAME: &str = "shiny gold";

#[derive(Debug)]
struct BagEntry<'a> {
    name: &'a str,
    count: usize,
}

impl<'a> BagEntry<'a> {
    fn new(name: &'a str, count: usize) -> Self {
        Self { name, count }
    }
}

fn parse_bags<'a>(input: &'a str) -> HashMap<&'a str, Vec<BagEntry<'a>>> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        let (key, values) = line.split_once(" bags contain ").unwrap();
        let values: Vec<_> = values
            .split(", ")
            .filter(|chunk| !chunk.starts_with("no"))
            .map(|chunk| {
                let (num, rest) = chunk.split_once(' ').unwrap();
                let num = num.parse().unwrap();
                let name_end_idx = rest.rfind(' ').unwrap();

                BagEntry::new(&rest[..name_end_idx], num)
            })
            .collect();

        bags.insert(key, values);
    }

    bags
}

fn calc_contain_count(input: &str) -> usize {
    let bags = parse_bags(input);

    let mut valid_bags = HashSet::from([TARGET_NAME]);

    let mut updated = true;

    while updated {
        updated = false;
        for (bag, children) in bags.iter() {
            if valid_bags.contains(bag) {
                continue;
            }

            for BagEntry {
                name: child_name, ..
            } in children
            {
                if valid_bags.contains(child_name) {
                    updated = true;
                    valid_bags.insert(bag);
                }
            }
        }
    }

    valid_bags.len() - 1
}

fn part_1() {
    let input = read_text_from_file("20", "07");
    let answer = calc_contain_count(&input);

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

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_solution() {
        assert_eq!(calc_contain_count(INPUT), 4);
    }
}

