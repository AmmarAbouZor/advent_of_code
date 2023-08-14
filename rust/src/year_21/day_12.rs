use std::collections::{HashMap, HashSet};

use crate::utls::read_text_from_file;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        match value {
            "start" => Node::Start,
            "end" => Node::End,
            cave => {
                if cave.chars().all(|ch| ch.is_ascii_lowercase()) {
                    Node::Small(cave.to_owned())
                } else {
                    Node::Big(cave.to_owned())
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct WayState {
    way: Vec<Node>,
    visited_small: HashSet<String>,
    used_duplicate_small: bool,
}

fn create_graph(input: &str) -> HashMap<Node, Vec<Node>> {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let (node_1, node_2) = line.split_once('-').unwrap();
        let from = Node::from(node_1);
        let to = Node::from(node_2);

        graph
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        // We need all nodes in the graph
        graph.entry(to).or_insert(Vec::new()).push(from);
    });

    graph
}

fn find_distinct_paths(input: &str, allow_one_duplicate: bool) -> usize {
    let graph = create_graph(input);

    let mut states: Vec<_> = graph
        .get(&Node::Start)
        .unwrap()
        .iter()
        .map(|node| {
            let way = vec![Node::Start, node.clone()];
            let mut visited_small = HashSet::new();
            if let Node::Small(name) = node {
                visited_small.insert(name.clone());
            }
            WayState {
                way,
                visited_small,
                used_duplicate_small: false,
            }
        })
        .collect();

    let mut ways_hash = HashSet::new();

    let mut ways_count = 0;

    while let Some(state) = states.pop() {
        let last_node = state.way.last().unwrap();

        if last_node == &Node::End {
            ways_count += 1;
            continue;
        }

        if last_node == &Node::Start {
            continue;
        }

        for node in graph.get(last_node).unwrap().iter() {
            let mut cloned_state = state.clone();

            cloned_state.way.push(node.clone());
            if !ways_hash.insert(cloned_state.way.clone()) {
                continue;
            }

            if let Node::Small(name) = node {
                if !cloned_state.visited_small.insert(name.clone()) {
                    if allow_one_duplicate && !cloned_state.used_duplicate_small {
                        cloned_state.used_duplicate_small = true;
                    } else {
                        continue;
                    }
                }
            }

            states.push(cloned_state);
        }
    }

    ways_count
}

fn part_1() {
    let input = read_text_from_file("21", "12");
    let answer = find_distinct_paths(&input, false);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "12");
    let answer = find_distinct_paths(&input, true);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const INPUT_2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const INPUT_3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part_1() {
        assert_eq!(find_distinct_paths(INPUT_1, false), 10);
        assert_eq!(find_distinct_paths(INPUT_2, false), 19);
        assert_eq!(find_distinct_paths(INPUT_3, false), 226);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(find_distinct_paths(INPUT_1, true), 36);
        assert_eq!(find_distinct_paths(INPUT_2, true), 103);
        assert_eq!(find_distinct_paths(INPUT_3, true), 3509);
    }
}
