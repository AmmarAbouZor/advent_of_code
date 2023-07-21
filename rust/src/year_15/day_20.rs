#![allow(warnings, unused)]

fn get_factors(num: u64) -> Vec<u64> {
    (1..num + 1).into_iter().filter(|&f| num % f == 0).collect()
}

fn get_house_score_no_filter(house_num: u64) -> u64 {
    get_factors(house_num).into_iter().sum::<u64>() * 10
}

fn get_house_score_with_filter(house_num: u64) -> u64 {
    get_factors(house_num)
        .into_iter()
        .filter(|num| house_num / num <= 50)
        .sum::<u64>()
        * 11
}

fn get_min_house<F>(target: u64, get_house_score: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut step_count = 0.0;
    let mut last_min_house = target;
    loop {
        step_count += 0.2;
        let mut count = target / (10.0 * step_count) as u64;
        let mut last_sum = 0;

        while last_sum < target {
            count += 1;

            last_sum = get_house_score(count);
        }

        println!("{step_count}: {count}");

        if last_min_house == count {
            return count;
        }

        last_min_house = count;
    }
}

fn part_1() {
    let house_num = get_min_house(36000000, get_house_score_no_filter);
    println!("part_1: lowest house num is {house_num}")
}

fn part_2() {
    let house_num = get_min_house(36000000, get_house_score_with_filter);
    println!("part_2: lowest house num is {house_num}")
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_min_house() {
        assert_eq!(10, get_house_score_no_filter(1));
        assert_eq!(30, get_house_score_no_filter(2));
        assert_eq!(40, get_house_score_no_filter(3));
        assert_eq!(70, get_house_score_no_filter(4));
        assert_eq!(60, get_house_score_no_filter(5));
        assert_eq!(120, get_house_score_no_filter(6));
        assert_eq!(80, get_house_score_no_filter(7));
        assert_eq!(150, get_house_score_no_filter(8));
        assert_eq!(130, get_house_score_no_filter(9));

        // with filter
        assert_eq!(781, get_house_score_with_filter(51));
    }
}
