use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
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
                .filter(|(_, &time)| time > i)
                .map(|(valve, _)| valve.rate)
                .sum::<usize>();
        }

        res
    }
}

fn calc_most_pressure(input: &str) -> usize {
    let valves: BTreeMap<String, Valve> = input
        .lines()
        .map(Valve::from)
        .map(|valva| (valva.name.clone(), valva))
        .collect();

    let func_valves_count = valves.values().filter(|va| va.rate > 0).count();

    let mut states = VecDeque::new();
    // let mut finished = BTreeSet::new();

    states.push_front((1, State::new(valves.get("AA").unwrap())));

    let mut result = 0;

    while let Some((mut time, mut state)) = states.pop_front() {
        // if let Some(index) = states.iter().position(|(_, s)| *s == state) {
        //     // println!("entered if");
        //     // dbg!(&states);
        //     // dbg!(&state);
        //     // println!("enter if");
        //     let t = states.iter().nth(index).unwrap().0;
        //     // dbg!((&t, &time));
        //     if t >= time {
        //         // println!("enter remove");
        //         // states.remove(index).unwrap();
        //     } else {
        //         // println!("skipped");
        //         continue;
        //     }
        // }
        // println!("skiped if");
        // dbg!(&states);
        // dbg!(&state);

        // println!("escape");

        if state.opened_valves.len() == func_valves_count {
            // dbg!(&state);
            // if finished.insert(state.clone()) {
            let curr_res = state.calc();
            result = result.max(curr_res);
            println!("{}-{}", curr_res, result);

            // }
        } else if time > 30 {
            // max time exceeded
        } else {
            time += 1;
            if state.current.rate > 0 && !state.opened_valves.contains_key(state.current) {
                state.opened_valves.insert(state.current, time);
                states.push_front((time, state));
            } else {
                for valve in state.current.lead_to.iter() {
                    state.current = valves.get(valve).unwrap();
                    states.push_front((time, state.clone()));
                }
            }
        }
    }

    // dbg!(finished);

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
