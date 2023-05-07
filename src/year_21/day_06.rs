use crate::utls::read_text_from_file;

fn fetch_input(input: &str) -> Vec<u8> {
    input
        .lines() // input always added break line at end if the input
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn calc_fish_nums(input: &str, limit: usize) -> usize {
    let mut fish = fetch_input(input);

    let mut count = 0;

    while count < limit {
        count += 1;
        let mut new_count = 0;
        fish.iter_mut().for_each(|num| {
            if *num == 0 {
                *num = 6;
                new_count += 1;
            } else {
                *num -= 1;
            }
        });

        fish.extend(vec![8; new_count]);
    }

    fish.len()
}

fn part_1() {
    let input = read_text_from_file("21", "06");
    let answer = calc_fish_nums(&input, 80);

    println!("Part 1 answer is {answer}");
}

fn calc_fish_nums_optimized(input: &str, limit: usize) -> usize {
    let fish = fetch_input(input);

    const FISH_CYCLE: usize = 6;
    const NEW_FISH_CYCLE: usize = 8;

    let mut fish_plan = vec![0; limit + NEW_FISH_CYCLE + 1];
    for num in fish.iter() {
        fish_plan[*num as usize] += 1;
    }

    let mut fish_count = fish.len();

    for counter in 0..limit {
        let new_fish = fish_plan[counter];
        // existed fish
        fish_plan[counter + FISH_CYCLE + 1] += new_fish;
        // new fish
        fish_plan[counter + NEW_FISH_CYCLE + 1] += new_fish;
        fish_count += new_fish;
    }

    fish_count
}

fn part_2() {
    let input = read_text_from_file("21", "06");
    let answer = calc_fish_nums_optimized(&input, 256);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"3,4,3,1,2";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_fish_nums(INPUT, 18), 26);
        assert_eq!(calc_fish_nums(INPUT, 80), 5934);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(calc_fish_nums_optimized(INPUT, 256), 26984457539);
    }
}
