use std::collections::HashMap;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Polymor {
    template: Vec<u8>,
    insert_map: HashMap<Vec<u8>, u8>,
}

impl From<&str> for Polymor {
    fn from(input: &str) -> Self {
        let (template, insertions) = input.split_once("\n\n").unwrap();

        let template = template.as_bytes().to_vec();

        let insert_map = insertions
            .lines()
            .map(|line| {
                let (pair, insertion) = line.split_once(" -> ").unwrap();

                (
                    pair.as_bytes().to_vec(),
                    *insertion.as_bytes().first().unwrap(),
                )
            })
            .collect();

        Polymor {
            template,
            insert_map,
        }
    }
}

impl Polymor {
    fn apply_step(&mut self) {
        let mut new_template = Vec::with_capacity(self.template.len() * 2);

        for pair in self.template.windows(2) {
            new_template.push(pair[0]);
            let insertion = *self.insert_map.get(pair).unwrap();
            new_template.push(insertion);
        }

        new_template.push(*self.template.last().unwrap());

        self.template = new_template;
    }

    fn calc_diff(&self) -> usize {
        let mut count_map = HashMap::new();

        for &ch in self.template.iter() {
            count_map
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let max = count_map.values().max().unwrap();
        let min = count_map.values().min().unwrap();

        max - min
    }
}

fn calc_diff(input: &str, count: usize) -> usize {
    let mut polymor = Polymor::from(input);

    for _ in 0..count {
        polymor.apply_step();
    }

    polymor.calc_diff()
}

fn part_1() {
    let input = read_text_from_file("21", "14");
    let answer = calc_diff(&input, 10);

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

    const INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_diff(INPUT, 10), 1588);
        assert_eq!(calc_diff(INPUT, 40), 2188189693529);
    }
}
