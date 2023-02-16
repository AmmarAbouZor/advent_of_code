use std::collections::{BTreeMap, BinaryHeap};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: usize,
    lead_to: Vec<String>,
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();

        let name = parts[1].to_owned();

        let rate = parts[4]
            .strip_prefix("rate=")
            .and_then(|txt| txt.strip_suffix(";"))
            .and_then(|txt| txt.parse().ok())
            .unwrap();

        let lead_to = parts[9..]
            .into_iter()
            .map(|txt| txt.trim_end_matches(',').to_owned())
            .collect();

        Valve {
            name,
            rate,
            lead_to,
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Pair<'a>(&'a str, &'a str);

fn calc_distances_floyd(valves: &Vec<Valve>) -> BTreeMap<Pair, usize> {
    let mut distances = BTreeMap::new();

    valves.iter().for_each(|src| {
        src.lead_to.iter().for_each(|target| {
            distances.insert(Pair(src.name.as_str(), target.as_str()), 1);
        })
    });

    let names: Vec<_> = valves.iter().map(|v| v.name.as_str()).collect();

    for mid in names.iter() {
        for src in names.iter() {
            for target in names.iter() {
                let Some(&src_mid) = distances.get(&Pair(src, mid)) else {continue};
                let Some(&mid_target) = distances.get(&Pair(mid, target)) else {continue};

                distances
                    .entry(Pair(src, target))
                    .and_modify(|val| *val = (*val).min(src_mid + mid_target))
                    .or_insert(src_mid + mid_target);
            }
        }
    }

    distances
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State<'a> {
    opened_valves: BTreeMap<&'a Valve, usize>,
    current: &'a Valve,
}

impl<'a> State<'a> {
    fn new(current: &'a Valve) -> Self {
        let opened_valves = BTreeMap::new();

        Self {
            opened_valves,
            current,
        }
    }

    fn calc(&self) -> usize {
        let mut res = 0;

        for i in 1..=30 {
            res += self
                .opened_valves
                .iter()
                .filter(|(_, &time)| time < i)
                .map(|(valve, _)| valve.rate)
                .sum::<usize>();
        }

        res
    }
}

fn calc_most_pressure(input: &str) -> usize {
    let valves: Vec<Valve> = input.lines().map(Valve::from).collect();

    let distances = calc_distances_floyd(&valves);

    let valves_with_rate: BTreeMap<String, Valve> = valves
        .iter()
        .cloned()
        .filter(|va| va.name == "AA" || va.rate > 0)
        .map(|valva| (valva.name.clone(), valva))
        .collect();

    let mut states = BinaryHeap::new();

    states.push((1, State::new(valves_with_rate.get("AA").unwrap())));

    let mut result = 0;

    while let Some((time, mut state)) = states.pop() {
        if time > 30 {
            let curr_res = state.calc();
            result = result.max(curr_res);
        } else {
            if state.current.rate > 0 && !state.opened_valves.contains_key(state.current) {
                state.opened_valves.insert(state.current, time);
                states.push((time + 1, state));
            } else {
                for valve in valves_with_rate
                    .values()
                    .filter(|val| !state.opened_valves.contains_key(val))
                {
                    let add_time = distances
                        .get(&Pair(state.current.name.as_str(), valve.name.as_str()))
                        .unwrap();
                    let mut state_cloned = state.clone();
                    state_cloned.current = valve;
                    states.push((time + add_time, state_cloned));
                }
            }
        }
    }

    result
}

fn part_1() {
    let input = read_text_from_file("22", "16");

    let answer = calc_most_pressure(&input);

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

    const INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_most_pressure(INPUT), 1651);
    }
}
