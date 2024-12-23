use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

fn parse(input: &'static str) -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let mut map = BTreeMap::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let (left, right) = line.split_once('-').unwrap();
        map.entry(left).or_insert_with(BTreeSet::new).insert(right);
        map.entry(right).or_insert_with(BTreeSet::new).insert(left);
    }

    map
}

fn get_sets_count(input: &'static str) -> usize {
    let map = parse(input);
    let mut sets: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();
    for (&first, f_set) in map.iter() {
        if !first.starts_with('t') {
            continue;
        }

        for &second in f_set {
            let Some(s_set) = map.get(second) else {
                continue;
            };

            for &third in s_set {
                if first != third && map[third].contains(first) {
                    let comb = BTreeSet::from_iter([first, second, third]);
                    assert_eq!(comb.len(), 3);
                    sets.insert(comb);
                }
            }
        }
    }

    sets.len()
}

fn part_1(input: &'static str) {
    let ans = get_sets_count(input);
    println!("Part 1 answer is {ans}");
}

fn get_largest_set(input: &'static str) -> String {
    fn fill_rec(
        current: &'static str,
        cur_set: BTreeSet<&'static str>,
        sets: &mut BTreeSet<BTreeSet<&'static str>>,
        map: &BTreeMap<&'static str, BTreeSet<&'static str>>,
        checked: &mut BTreeSet<BTreeSet<&'static str>>,
    ) {
        if checked.contains(&cur_set) {
            return;
        }

        checked.insert(cur_set.clone());
        for next in map[current].iter() {
            let mut clone = cur_set.clone();

            if clone.insert(next) {
                let all_connected = cur_set.iter().all(|p| map[next].contains(p));
                if all_connected {
                    // New item
                    fill_rec(next, clone, sets, map, checked);
                }
            } else {
                if clone.len() == 2 {
                    continue;
                }

                // Connection check is already done before calling fill_rec again.

                sets.insert(clone);
            }
        }
    }

    let map = parse(input);
    let mut sets: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();
    let mut checked: BTreeSet<BTreeSet<&'static str>> = BTreeSet::new();
    for &first in map.keys() {
        fill_rec(
            first,
            BTreeSet::from_iter([first]),
            &mut sets,
            &map,
            &mut checked,
        );
    }

    let last: Vec<_> = sets
        .into_iter()
        .sorted_by_key(|s| s.len())
        .last()
        .unwrap()
        .into_iter()
        .collect();

    last.join(",")
}

fn part_2(input: &'static str) {
    let ans = get_largest_set(input);
    println!("Part 2 answer is '{ans}'");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "23").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_solution() {
        let ans = get_sets_count(INPUT);
        assert_eq!(ans, 7);

        let ans2 = get_largest_set(INPUT);
        assert_eq!(&ans2, "co,de,ka,ta")
    }
}

