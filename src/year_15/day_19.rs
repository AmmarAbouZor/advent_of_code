#![allow(warnings, unused)]

use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

use crate::utls::read_lines_from_file;

#[derive(Debug, Clone)]
struct Replacement {
    base: String,
    result: String,
}

impl From<&String> for Replacement {
    fn from(line: &String) -> Self {
        let mut parts = line.split(" => ");
        let base = parts.next().unwrap().to_owned();
        let result = parts.next().unwrap().to_owned();

        Replacement { base, result }
    }
}

fn fetch_input() -> (Vec<Replacement>, String) {
    let mut lines = read_lines_from_file(r"src/year_15/day_19.txt");
    let replacements = lines
        .iter()
        .take(lines.len() - 2)
        .map(|line| line.into())
        .collect();

    let input = lines.pop().unwrap();

    (replacements, input)
}

fn apply_replacement(input: &String, repl: Replacement, molecules: &mut HashSet<String>) {
    input
        .match_indices(repl.base.as_str())
        .for_each(|(index, base)| {
            let mut molecule = input.clone();
            molecule.replace_range(index..index + base.len(), &repl.result);
            molecules.insert(molecule);
        })
}

fn calc_molecules_cout(input: &String, replacements: Vec<Replacement>) -> usize {
    let mut hash = HashSet::new();
    replacements.into_iter().for_each(|repl| {
        apply_replacement(&input, repl, &mut hash);
    });

    hash.len()
}

fn part_1() {
    let (replacements, input) = fetch_input();
    let molecule_count = calc_molecules_cout(&input, replacements);

    println!("distinct molecule count is {molecule_count}");
}

fn reverse_replacement(repl: &Replacement, molecule: &mut String) -> bool {
    if let Some(index) = molecule.find(&repl.result) {
        molecule.replace_range(index..index + repl.result.len(), &repl.base);
        true
    } else {
        false
    }
}

fn find_any_count_rand(input: String, mut replacements: Vec<Replacement>) -> usize {
    let mut found = false;
    while !found {
        let mut cloned_input = input.clone();
        let mut count = 0;
        let mut done = false;
        while !done {
            done = true;
            for repl in replacements.iter() {
                if reverse_replacement(repl, &mut cloned_input) {
                    count += 1;
                    done = false;
                }
            }
        }

        if cloned_input.len() == 1 {
            return count;
        } else {
            replacements.shuffle(&mut thread_rng());
        }
    }

    panic!("Nothing found");
}

fn part_2() {
    let (replacements, input) = fetch_input();
    let count = find_any_count_rand(input, replacements);

    println!("resloved in {count} moves");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_replacements(part: u8) -> Vec<Replacement> {
        let input = match part {
            1 => {
                r"H => HO
H => OH
O => HH"
            }
            _ => {
                r"e => H
e => O
H => HO
H => OH
O => HH"
            }
        };
        input
            .lines()
            .map(|line| (&line.to_string()).into())
            .collect()
    }
    #[test]
    fn test_part_one() {
        let replacements = get_test_replacements(1);
        let input = "HOH".to_owned();

        assert_eq!(calc_molecules_cout(&input, replacements), 4);
    }

    #[test]
    fn test_part_two() {
        let replacements = get_test_replacements(2);
        let input = "HOH".to_owned();

        assert_eq!(find_any_count_rand(input, replacements), 3);
    }
}
