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

fn calc_nested_count(input: &str) -> usize {
    let bags_map = parse_bags(input);

    get_children_count(TARGET_NAME, &bags_map) - 1
}

fn get_children_count(bag: &str, bags_map: &HashMap<&str, Vec<BagEntry>>) -> usize {
    bags_map
        .get(bag)
        .map(|bags| {
            bags.iter()
                .map(|bag| bag.count * get_children_count(bag.name, bags_map))
                .sum::<usize>()
                + 1
        })
        .unwrap()
}

fn part_1() {
    let input = read_text_from_file("20", "07");
    let answer = calc_contain_count(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "07");
    let answer = calc_nested_count(&input);

    println!("Part 2 answer i {answer}");
}

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

    const INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_solution() {
        assert_eq!(calc_contain_count(INPUT), 4);
        assert_eq!(calc_nested_count(INPUT), 32);
        assert_eq!(calc_nested_count(INPUT_2), 126);
    }
}

