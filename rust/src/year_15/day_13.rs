#![allow(warnings, unused)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utls::read_lines_from_file;

const MY_NAME: &str = "Me";

#[derive(Debug, Hash, PartialEq, Eq)]
struct Couple {
    a_person: String,
    b_person: String,
}

impl Couple {
    fn new(a_person: &str, b_person: &str) -> Self {
        Self {
            a_person: a_person.into(),
            b_person: b_person.into(),
        }
    }
}

fn fetch_input() -> (HashMap<Couple, i32>, HashSet<String>) {
    let mut map = HashMap::new();
    let mut people = HashSet::new();
    read_lines_from_file(r"src/year_15/day_13.txt")
        .iter()
        .for_each(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let a_person = parts[0];
            let b_person = parts.last().unwrap().trim_end_matches('.');
            let score = {
                let num: i32 = parts[3].parse().unwrap();
                if parts[2] == "gain" { num } else { -num }
            };
            map.insert(Couple::new(a_person, b_person), score);
            people.insert(a_person.into());
            people.insert(b_person.into());
        });
    (map, people)
}

fn get_two_person_score(per_a: &str, per_b: &str, map: &HashMap<Couple, i32>) -> i32 {
    let score_1 = map.get(&Couple::new(per_a, per_b)).unwrap();
    let score_2 = map.get(&Couple::new(per_b, per_a)).unwrap();
    score_1 + score_2
}

fn calc_score(arrangement: Vec<&String>, map: &HashMap<Couple, i32>) -> i32 {
    let mut score = get_two_person_score(
        arrangement.first().unwrap(),
        arrangement.last().unwrap(),
        map,
    );
    for i in 0..arrangement.len() - 1 {
        score += get_two_person_score(arrangement[i], arrangement[i + 1], map);
    }

    score
}

fn part_1() {
    let (map, people) = fetch_input();
    let highest_score = people
        .iter()
        .permutations(people.len())
        .unique()
        .map(|arrangement| calc_score(arrangement, &map))
        .max()
        .unwrap();

    println!("highest_score is {highest_score}");
}

fn add_me_to_map(map: &mut HashMap<Couple, i32>, people: &HashSet<String>) {
    for person in people {
        map.insert(Couple::new(MY_NAME, person.as_str()), 0);
        map.insert(Couple::new(person.as_str(), MY_NAME), 0);
    }
}

fn part_2() {
    let (mut map, mut people) = fetch_input();

    add_me_to_map(&mut map, &people);
    people.insert(MY_NAME.into());

    let highest_score = people
        .iter()
        .permutations(people.len())
        .unique()
        .map(|arrangement| calc_score(arrangement, &map))
        .max()
        .unwrap();

    println!("highest_score with me is {highest_score}");
}

pub fn run() {
    part_1();
    part_2();
}
