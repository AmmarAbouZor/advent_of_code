use std::collections::HashMap;

use itertools::Itertools;

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
    // These two methods are the direct way to do it as the instructions but this can't handle
    // large number of iterations. This work until 30 iterations
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

    /// We need to keep track on the numbers of occurrences of the each pair.
    /// In each step we update these occurrences then we take the first characters of the pairs
    /// Then we need to add one to the last characters from the initial template
    fn calc_with_pair_count(&self, steps: usize) -> usize {
        let mut pair_count = self
            .template
            .windows(2)
            .map(|slice| slice.to_vec())
            .counts();

        for _ in 0..steps {
            let mut next_count = HashMap::new();
            for (slice, count) in pair_count.iter() {
                let insertion = *self.insert_map.get(slice).unwrap();
                *next_count.entry(vec![slice[0], insertion]).or_insert(0) += count;
                *next_count.entry(vec![insertion, slice[1]]).or_insert(0) += count;
            }

            pair_count = next_count;
        }

        let mut singls_count = HashMap::new();

        for (slice, count) in pair_count.into_iter() {
            *singls_count.entry(slice[0]).or_insert(0) += count;
        }

        // Last Chars in the initial template
        *singls_count
            .entry(*self.template.last().unwrap())
            .or_insert(0) += 1;

        let max = singls_count.values().max().unwrap();
        let min = singls_count.values().min().unwrap();

        max - min
    }
}

/// This is the direct way to do it as the instructions but this can't handle
/// large number of iterations. This work until 30 iterations
fn calc_diff_basic_10(input: &str) -> usize {
    let mut polymor = Polymor::from(input);

    for _ in 0..10 {
        polymor.apply_step();
    }

    polymor.calc_diff()
}

fn calc_diff_advance(input: &str, steps: usize) -> usize {
    let polymor = Polymor::from(input);

    polymor.calc_with_pair_count(steps)
}

fn part_1() {
    let input = read_text_from_file("21", "14");
    let answer = calc_diff_basic_10(&input);

    println!("Part 1 answer is {answer}");
}
fn part_2() {
    let input = read_text_from_file("21", "14");
    let answer = calc_diff_advance(&input, 40);

    println!("Part 2 answer is {answer}");
}

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
        assert_eq!(calc_diff_basic_10(INPUT), 1588);
    }

    #[test]
    fn test_all_parts_advanced() {
        assert_eq!(calc_diff_advance(INPUT, 10), 1588);
        assert_eq!(calc_diff_advance(INPUT, 40), 2188189693529);
    }
}
