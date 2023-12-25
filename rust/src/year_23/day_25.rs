use std::collections::{HashMap, HashSet, VecDeque};

fn parse_edges(input: &'static str) -> HashMap<&'static str, HashSet<&'static str>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (key, parts) = line.split_once(": ").unwrap();

        for part in parts.split_whitespace() {
            // Key -> Part
            map.entry(key)
                .or_insert_with(|| HashSet::new())
                .insert(part);

            // Connections are non directional -> Add the connection from part to key too
            map.entry(part)
                .or_insert_with(|| HashSet::new())
                .insert(key);
        }
    }

    map
}

fn get_uniqe_key(n1: &'static str, n2: &'static str) -> [&'static str; 2] {
    if n1 > n2 {
        [n1, n2]
    } else {
        [n2, n1]
    }
}

fn solve(input: &'static str) -> usize {
    let edges = parse_edges(input);
    // Let's assume that the target cuts are the connections with the highest frequency
    // This assumtion is valid for the real input but not for the example

    let mut freq_map = HashMap::new();

    for &edge in edges.keys() {
        let mut queue = VecDeque::new();
        queue.push_back(edge);
        let mut visited = HashSet::new();

        while let Some(node) = queue.pop_front() {
            for next in edges.get(node).unwrap() {
                if !visited.insert(next) {
                    continue;
                }

                let key = get_uniqe_key(node, next);

                freq_map
                    .entry(key)
                    .and_modify(|c| *c += 1)
                    .or_insert(1_usize);

                queue.push_back(next);
            }
        }
    }

    let mut map_vec: Vec<_> = freq_map.iter().collect();
    map_vec.sort_unstable_by_key(|e| e.1);
    let cutted: Vec<_> = map_vec.iter().rev().take(3).map(|p| *p.0).collect();

    // find the count from any edge with cutting the three links
    let start = *edges.keys().next().unwrap();
    let mut size = 1;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        for &next in edges.get(node).unwrap() {
            let key = get_uniqe_key(node, next);

            if cutted.contains(&key) {
                continue;
            }

            if visited.insert(next) {
                size += 1;
                queue.push_back(next);
            }
        }
    }

    size * (edges.len() - size)
}

fn part_1(input: &'static str) {
    let answer = solve(input);

    println!("Par 1 answer is {answer}");
}

fn part_2(_input: &'static str) {
    println!("No part 2 on day 25. Year 23 is DONE!!!");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("23", "25").leak();
    part_1(input);
    part_2(input);
}

