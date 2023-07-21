use std::collections::BTreeMap;

use itertools::Itertools;

use crate::utls::read_text_from_file;

#[derive(Debug, Hash, Clone, Ord, PartialOrd, PartialEq, Eq)]
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
            .and_then(|txt| txt.strip_suffix(';'))
            .and_then(|txt| txt.parse().ok())
            .unwrap();

        let lead_to = parts[9..]
            .iter()
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

fn calc_distances_floyd(valves: &[Valve]) -> BTreeMap<Pair, usize> {
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

fn simulate(input: &str, minutes: usize) -> (usize, BTreeMap<u64, usize>) {
    let valves: Vec<Valve> = input.lines().map(Valve::from).collect();

    let distances = calc_distances_floyd(&valves);

    let valves_with_rate_idx: BTreeMap<String, usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, va)| va.name == "AA" || va.rate > 0)
        .map(|(idx, valv)| (valv.name.to_owned(), idx))
        .collect();

    let mut mask_flow: BTreeMap<u64, usize> = BTreeMap::new();

    let flow = traveling_salesman(
        &valves,
        &mut mask_flow,
        &valves_with_rate_idx,
        &distances,
        0,
        minutes,
        0,
        "AA".into(),
    );

    (flow, mask_flow)
}

#[allow(clippy::too_many_arguments)]
fn traveling_salesman(
    valves: &Vec<Valve>,
    mask_flows: &mut BTreeMap<u64, usize>,
    valves_with_rate_idx: &BTreeMap<String, usize>,
    distances: &BTreeMap<Pair, usize>,
    mask: u64,
    minutes: usize,
    flow: usize,
    current_valve: String,
) -> usize {
    let mut max = flow;

    mask_flows
        .entry(mask)
        .and_modify(|fl| *fl = (*fl).max(flow))
        .or_insert(flow);

    for target in valves_with_rate_idx.keys() {
        let new_minutes = minutes
            .checked_sub(*distances.get(&Pair(&current_valve, target)).unwrap())
            .and_then(|m| m.checked_sub(1))
            .unwrap_or(0);

        let valve_idx = *valves_with_rate_idx.get(target).unwrap();

        let sub_mask = 1 << valve_idx;

        if new_minutes == 0 || (mask & sub_mask) != 0 {
            continue;
        }

        let new_mask = mask | sub_mask;

        let new_flow = flow + (new_minutes * valves[valve_idx].rate);

        let new_max = traveling_salesman(
            valves,
            mask_flows,
            valves_with_rate_idx,
            distances,
            new_mask,
            new_minutes,
            new_flow,
            target.to_owned(),
        );

        max = max.max(new_max);
    }

    max
}

fn calc_most_pressure(input: &str) -> usize {
    let (flow, _) = simulate(input, 30);

    flow
}

fn calc_most_pressure_two(input: &str) -> usize {
    let (_, mask_flow) = simulate(input, 26);

    let mut flow = 0;

    mask_flow.iter().combinations(2).for_each(|comb| {
        let (me_mask, me_score) = comb[0];
        let (elf_mask, elf_score) = comb[1];

        if (me_mask & elf_mask) == 0 {
            flow = flow.max(me_score + elf_score);
        }
    });

    flow
}

fn part_1() {
    let input = read_text_from_file("22", "16");

    let answer = calc_most_pressure(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "16");

    let answer = calc_most_pressure_two(&input);

    println!("Part 2 answer is {answer}");
}

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

    #[test]
    fn test_part_2() {
        assert_eq!(calc_most_pressure_two(INPUT), 1707);
    }
}
