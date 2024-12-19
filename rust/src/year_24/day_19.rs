use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse(input: &'static str) -> (Vec<&'static str>, Vec<&'static str>) {
    let (strips, designs) = input.split_once("\n\n").unwrap();
    let strips = strips.split(", ").collect();
    let designs = designs.lines().filter(|s| !s.is_empty()).collect();

    (strips, designs)
}

fn count_possiblities(design: &'static str, strips: &[&'static str]) -> usize {
    fn count_rec(
        design: &'static str,
        strips: &[&'static str],
        memo: &mut HashMap<&'static str, usize>,
    ) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(ways) = memo.get(design) {
            return *ways;
        }

        let mut ways = 0;
        for strip in strips {
            if let Some(remain) = design.strip_prefix(strip) {
                ways += count_rec(remain, strips, memo);
            }
        }

        assert!(memo.insert(design, ways).is_none());

        ways
    }

    let mut memo = HashMap::new();
    count_rec(design, strips, &mut memo)
}

fn calc_valid(input: &'static str) -> usize {
    let (strips, designs) = parse(input);

    designs
        .par_iter()
        .filter(|d| count_possiblities(d, &strips) > 0)
        .count()
}

fn calc_all(input: &'static str) -> usize {
    let (strips, designs) = parse(input);

    designs
        .par_iter()
        .map(|d| count_possiblities(d, &strips))
        .sum()
}

fn part_1(input: &'static str) {
    let ans = calc_valid(input);
    println!("Part 1 answer is {ans}");
}

fn part_2(input: &'static str) {
    let ans = calc_all(input);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "19").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_solution() {
        let valid = calc_valid(INPUT);
        assert_eq!(valid, 6);

        let counts = calc_all(INPUT);
        assert_eq!(counts, 16);
    }
}
