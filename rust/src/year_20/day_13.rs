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

    ids.trim()
        .split(',')
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

    let exec_time = end_time - start_time;
    let exec_time = exec_time.as_nanos();
    println!("Part 1 answer is {answer_1}. It took {exec_time}");
}

// Solution is looked up online and imported from python, since it's about numbers algorithms
fn calc_earliest_matching(input: &str) -> u128 {
    let (_target, buses) = input.split_once('\n').unwrap();
    let buses: Vec<(u128, u128)> = buses
        .trim()
        .split(',')
        .enumerate()
        .filter_map(|(idx, num)| num.parse().ok().map(|num| ((idx as u128), num)))
        .collect();

    let mut lcm = 1;
    let mut time = 0;
    for i in 0..buses.len() - 1 {
        let bus = buses[i + 1].1;
        let idx = buses[i + 1].0;
        lcm *= buses[i].1;
        while (time + idx) % bus != 0 {
            time += lcm;
        }
    }

    time
}

fn part_2(input: &str) {
    let answer_2 = calc_earliest_matching(input);

    println!("Part 2 answer is {answer_2}");
}

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
    fn test_part() {
        assert_eq!(get_earliest(INPUT), 295);
        let test_records = [
            (INPUT, 1068781),
            ("1\n17,x,13,19", 3417),
            ("1\n67,7,59,61", 754018),
            ("1\n67,x,7,59,61", 779210),
            ("1\n67,7,x,59,61", 1261476),
            ("1\n1789,37,47,1889", 1202161486),
        ];
        for (input, result) in test_records {
            assert_eq!(calc_earliest_matching(input), result);
        }
    }
}
