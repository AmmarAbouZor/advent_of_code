use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn get_win_count(&self) -> usize {
        (1..self.time)
            .map(|hold| hold * (self.time - hold))
            .filter(|dist| *dist > self.distance)
            .count()
    }
}

fn get_win_product(races: &[Race]) -> usize {
    races.iter().map(|race| race.get_win_count()).product()
}

fn part_1(input: &[Race]) {
    let answer = get_win_product(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &[Race]) {}

pub fn run() {
    let input = [
        Race::new(48, 261),
        Race::new(93, 1192),
        Race::new(84, 1019),
        Race::new(66, 1063),
    ];
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        let input = [Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        assert_eq!(get_win_product(&input), 288);
    }
}

