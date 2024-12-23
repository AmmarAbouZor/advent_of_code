use std::collections::{BTreeMap, BTreeSet};

fn parse(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut map = BTreeMap::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let (left, right) = line.split_once('-').unwrap();
        map.entry(left)
            .or_insert_with(|| BTreeSet::new())
            .insert(right);
        map.entry(right)
            .or_insert_with(|| BTreeSet::new())
            .insert(left);
    }

    map
}

fn get_sets_count(input: &str) -> usize {
    let map = parse(input);
    let mut sets: BTreeSet<BTreeSet<&str>> = BTreeSet::new();
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

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "23").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
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
    }
}

