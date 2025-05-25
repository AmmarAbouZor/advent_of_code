#![allow(warnings, unused)]

use std::collections::HashMap;

use crate::utls::read_lines_from_file;

#[derive(Debug, PartialEq)]
enum Compound {
    Children(u8),
    Cats(u8),
    Samoyeds(u8),
    Pomeranians(u8),
    Akitas(u8),
    Vizslas(u8),
    Goldfish(u8),
    Trees(u8),
    Cars(u8),
    Perfumes(u8),
}

impl Compound {
    fn match_for_part_2(&self, ticker_other: &Self) -> bool {
        match (self, ticker_other) {
            (&Compound::Children(me), &Compound::Children(other)) => me == other,
            (&Compound::Cats(me), &Compound::Cats(other)) => me > other,
            (&Compound::Samoyeds(me), &Compound::Samoyeds(other)) => me == other,
            (&Compound::Pomeranians(me), &Compound::Pomeranians(other)) => me < other,
            (&Compound::Akitas(me), &Compound::Akitas(other)) => me == other,
            (&Compound::Vizslas(me), &Compound::Vizslas(other)) => me == other,
            (&Compound::Goldfish(me), &Compound::Goldfish(other)) => me < other,
            (&Compound::Trees(me), &Compound::Trees(other)) => me > other,
            (&Compound::Cars(me), &Compound::Cars(other)) => me == other,
            (&Compound::Perfumes(me), &Compound::Perfumes(other)) => me == other,
            _ => false,
        }
    }
}

impl From<[&str; 2]> for Compound {
    fn from(parts: [&str; 2]) -> Self {
        let score: u8 = parts[1].trim_end_matches(',').parse().unwrap();

        match parts[0].trim_end_matches(':') {
            "children" => Compound::Children(score),
            "cats" => Compound::Cats(score),
            "samoyeds" => Compound::Samoyeds(score),
            "pomeranians" => Compound::Pomeranians(score),
            "akitas" => Compound::Akitas(score),
            "vizslas" => Compound::Vizslas(score),
            "goldfish" => Compound::Goldfish(score),
            "trees" => Compound::Trees(score),
            "cars" => Compound::Cars(score),
            "perfumes" => Compound::Perfumes(score),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Sue {
    id: u16,
    compound: [Compound; 3],
}

impl From<String> for Sue {
    fn from(line: String) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        let id: u16 = parts[1].trim_end_matches(':').parse().unwrap();
        let compound = [
            Compound::from([parts[2], parts[3]]),
            Compound::from([parts[4], parts[5]]),
            Compound::from([parts[6], parts[7]]),
        ];

        Sue { id, compound }
    }
}

const TICKER_TAP: [Compound; 10] = [
    Compound::Children(3),
    Compound::Cats(7),
    Compound::Samoyeds(2),
    Compound::Pomeranians(3),
    Compound::Akitas(0),
    Compound::Vizslas(0),
    Compound::Goldfish(5),
    Compound::Trees(3),
    Compound::Cars(2),
    Compound::Perfumes(1),
];

fn fetch_input() -> Vec<Sue> {
    read_lines_from_file(r"src/year_15/day_16.txt")
        .into_iter()
        .map(|line| line.into())
        .collect()
}

fn get_matches<F>(sues: Vec<Sue>, filter: F) -> HashMap<u16, u8>
where
    F: Fn(&Compound) -> bool,
{
    sues.into_iter()
        .map(|sue| {
            (
                sue.id,
                sue.compound.iter().filter(|comp| filter(comp)).count() as u8,
            )
        })
        .collect()
}

fn part_1() {
    let sues = fetch_input();
    let matches = get_matches(sues, |compound| TICKER_TAP.contains(compound));
    matches.iter().filter(|&(_, &v)| v == 3).for_each(|(k, _)| {
        println!("part 1: sue number {}, got 3 matches", *k);
    });
}

fn part_2() {
    let sues = fetch_input();
    let matches = get_matches(sues, |compound| {
        TICKER_TAP
            .iter()
            .any(|ticker| compound.match_for_part_2(ticker))
    });
    matches.iter().filter(|&(_, &v)| v == 3).for_each(|(k, _)| {
        println!("part 2: sue number {}, got 3 matches", *k);
    });
}

pub fn run() {
    part_1();
    part_2();
}
