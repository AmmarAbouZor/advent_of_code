use std::collections::BTreeMap;

use crate::utls::read_text_from_file;

fn get_diffs_product(input: &str) -> usize {
    let mut nums: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    nums.sort_unstable();

    let mut count_1 = 0;
    let mut count_3 = 0;
    let mut current = 0;

    for num in nums {
        match num - current {
            1 => count_1 += 1,
            2 => {}
            3 => count_3 += 1,
            _ => unreachable!("Diff can't be greater than 3"),
        }
        current = num;
    }

    count_3 += 1;

    count_1 * count_3
}

fn get_arrangement_count(input: &str) -> usize {
    let mut nums: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    nums.sort_unstable();

    let mut ways_set = BTreeMap::new();
    ways_set.insert(0, 1);

    for num in nums.iter() {
        let ways_count = *ways_set.get(&(num - 1)).unwrap_or(&0)
            + *ways_set.get(&(num.wrapping_sub(2))).unwrap_or(&0)
            + *ways_set.get(&(num.wrapping_sub(3))).unwrap_or(&0);

        assert!(ways_set.insert(*num, ways_count).is_none());
    }

    *ways_set.get(nums.last().unwrap()).unwrap()
}

fn part_1() {
    let input = read_text_from_file("20", "10");
    let answer = get_diffs_product(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "10");
    let answer = get_arrangement_count(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part_1() {
        assert_eq!(get_diffs_product(INPUT_1), 7 * 5);
        assert_eq!(get_diffs_product(INPUT_2), 22 * 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_arrangement_count(INPUT_1), 8);
        assert_eq!(get_arrangement_count(INPUT_2), 19208);
    }
}

