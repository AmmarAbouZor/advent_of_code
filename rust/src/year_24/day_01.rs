use itertools::Itertools;

fn parse_input(input: &'static str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once("   ").unwrap();
        left.push(n1.parse().unwrap());
        right.push(n2.parse().unwrap());
    }

    (left, right)
}

fn calc_total_distance(input: &'static str) -> usize {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip_eq(right.into_iter())
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum()
}

fn part_1(input: &'static str) {
    let dist = calc_total_distance(input);
    println!("Total distance is {dist}");
}

fn calc_similarity(input: &'static str) -> usize {
    let (left, right) = parse_input(input);
    left.into_iter()
        .map(|num| {
            let appearance = right.iter().filter(|&&n| n == num).count();
            num * appearance
        })
        .sum()
}

fn part_2(input: &'static str) {
    let simil = calc_similarity(input);
    println!("Similarity is {simil}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "01").leak();
    // let input = crate::include_input!("24", "01");
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_solution() {
        let dist = calc_total_distance(INPUT);
        assert_eq!(dist, 11);

        let similariy = calc_similarity(INPUT);
        assert_eq!(similariy, 31);
    }
}
