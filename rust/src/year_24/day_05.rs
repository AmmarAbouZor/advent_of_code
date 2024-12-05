use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn parse(input: &'static str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    let mut deps = HashMap::new();

    for line in part1.lines() {
        let (n1, n2) = line.split_once('|').unwrap();
        let n1 = n1.parse().unwrap();
        let n2 = n2.parse().unwrap();

        deps.entry(n1).or_insert(Vec::new()).push(n2);
    }

    let records = part2
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (deps, records)
}

fn valid_sum(input: &'static str) -> usize {
    let (deps, records) = parse(input);

    records
        .par_iter()
        .filter(|r| is_record_valid(r, &deps))
        .map(|r| {
            let mid = r.len() / 2;
            r[mid]
        })
        .sum()
}

fn is_record_valid(record: &[usize], deps: &HashMap<usize, Vec<usize>>) -> bool {
    for (idx, current) in record.iter().enumerate().take(record.len() - 1) {
        if record[idx + 1..]
            .iter()
            .any(|n| deps.get(n).is_some_and(|d| d.contains(current)))
        {
            return false;
        }
    }

    true
}

fn corrected_sum(input: &'static str) -> usize {
    let (deps, records) = parse(input);

    let invalid: Vec<_> = records
        .par_iter()
        .filter(|r| !is_record_valid(r, &deps))
        .cloned()
        .collect();

    invalid
        .into_par_iter()
        .map(|mut r| {
            correct_record(r.as_mut_slice(), &deps);
            r
        })
        .map(|r| {
            let mid = r.len() / 2;
            r[mid]
        })
        .sum()
}

fn correct_record(record: &mut [usize], deps: &HashMap<usize, Vec<usize>>) {
    let mut search = true;
    while search {
        search = false;
        for idx in 0..record.len() - 1 {
            if let Some(invalid) = record[idx + 1..]
                .iter()
                .position(|n| deps.get(n).is_some_and(|d| d.contains(&record[idx])))
            {
                let invalid = invalid + idx + 1;
                record.swap(idx, invalid);
                search = true;
                break;
            }
        }
    }
}

fn part_1(input: &'static str) {
    let sum = valid_sum(input);
    println!("Part 1 answer is {sum}")
}

fn part_2(input: &'static str) {
    let sum = corrected_sum(input);
    println!("Part 2 asnwer is {sum}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "05").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_solution() {
        let sum = valid_sum(INPUT);
        assert_eq!(sum, 143);

        let correct = corrected_sum(INPUT);
        assert_eq!(correct, 123);
    }
}

