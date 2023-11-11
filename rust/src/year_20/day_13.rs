use std::time::Instant;

use crate::utls::read_text_from_file;

fn get_min_diff(id: usize, target: usize) -> usize {
    let num_before = target / id;
    let first_arrive = id * (num_before + 1);

    first_arrive - target
}

fn get_earliest(input: &str) -> usize {
    let (target, ids) = input.split_once('\n').unwrap();
    let target: usize = target.parse().unwrap();

    ids.split(',')
        .flat_map(|id| id.parse().ok())
        .map(|id| (get_min_diff(id, target), id))
        .min()
        .map(|(diff, id)| diff * id)
        .unwrap()
}

fn part_1(input: &str) {
    let start_time = Instant::now();
    let answer_1 = get_earliest(input);
    let end_time = Instant::now();

    let heap_time = end_time - start_time;
    let heap_time = heap_time.as_nanos();
    println!("Part 1 answer is {answer_1}. It took {heap_time}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("20", "13");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part_1() {
        assert_eq!(get_earliest(INPUT), 295);
    }
}
