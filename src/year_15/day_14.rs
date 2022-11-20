#![allow(warnings, unused)]

use std::collections::HashMap;

use crate::utls::read_lines_from_file;

#[derive(Debug)]
struct Horse {
    name: String,
    speed: u32,
    running_period: u32,
    rest_period: u32,
}

impl Horse {
    fn calc_distance(&self, time: u32) -> u32 {
        let cycle = self.running_period + self.rest_period;
        let remaining = time % cycle;
        let whole_cicles_count = time / cycle;
        let running_seconds_in_remaining = if remaining > self.running_period {
            self.running_period
        } else {
            remaining
        };
        let running_seconds_in_full_cycles = whole_cicles_count * self.running_period;

        self.speed * (running_seconds_in_remaining + running_seconds_in_full_cycles)
    }
}

impl From<&str> for Horse {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(' ').collect();
        let name = parts[0].into();
        let speed: u32 = parts[3].parse().unwrap();
        let running_period: u32 = parts[6].parse().unwrap();
        let rest_period: u32 = parts[13].parse().unwrap();

        Horse {
            name,
            speed,
            running_period,
            rest_period,
        }
    }
}

fn get_horses() -> Vec<Horse> {
    let horses: Vec<Horse> = read_lines_from_file(r"src/year_15/day_14.txt")
        .iter()
        .map(|line| line.as_str().into())
        .collect();

    horses
}
const GOAL: u32 = 2503;
fn part_1() {
    let horses = get_horses();

    let max_distance = horses
        .iter()
        .map(|horse| horse.calc_distance(GOAL))
        .max()
        .unwrap();

    println!("max distance is {max_distance}");
}

fn part_2() {
    let horses = get_horses();
    let mut scores_map: HashMap<String, u32> = HashMap::new();

    for sec in 1..=GOAL {
        let current_scores: HashMap<String, u32> = horses
            .iter()
            .map(|horse| (horse.name.to_owned(), horse.calc_distance(sec)))
            .collect();
        let max = current_scores.values().max().unwrap();
        current_scores
            .iter()
            .filter(|score| score.1 == max)
            .for_each(|score| {
                let points = scores_map.entry(score.0.into()).or_insert(0);
                *points += 1;
            })
    }

    let max_points = scores_map.values().max().unwrap();

    println!("max points are {max_points}");
}

pub fn run() {
    part_1();
    part_2();
}
