use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Card {
    winners: Vec<usize>,
    nums: Vec<usize>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (_, all_nums) = line.split_once(": ").unwrap();
        let (winners, nums) = all_nums.split_once(" | ").unwrap();
        let winners = winners
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let nums = nums
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        Card { winners, nums }
    }
}

impl Card {
    fn calc_score(&self) -> usize {
        let winner_nums = self
            .nums
            .iter()
            .filter(|num| self.winners.contains(num))
            .count();
        if winner_nums > 0 {
            (0..winner_nums - 1).fold(1, |acc, _| acc * 2)
        } else {
            0
        }
    }
}

fn calc_total_score(input: &str) -> usize {
    input
        .lines()
        .map(Card::from)
        .map(|card| card.calc_score())
        .sum()
}

fn part_1(input: &str) {
    let answer_1 = calc_total_score(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {}

pub fn run() {
    let input = read_text_from_file("23", "04");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solution() {
        assert_eq!(calc_total_score(INPUT), 13)
    }
}

