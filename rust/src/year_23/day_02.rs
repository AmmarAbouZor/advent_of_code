use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug)]
struct CubesEntry {
    color: Color,
    count: usize,
}

impl From<&str> for CubesEntry {
    fn from(value: &str) -> Self {
        let (count, color) = value.trim().split_once(' ').unwrap();
        let count = count.parse().unwrap();
        let color = match color {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            invalid => unreachable!("Invalid input: '{invalid}'"),
        };

        CubesEntry { color, count }
    }
}

impl CubesEntry {
    fn is_valid(&self) -> bool {
        match self.color {
            Color::Blue => self.count <= 14,
            Color::Red => self.count <= 12,
            Color::Green => self.count <= 13,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Vec<CubesEntry>>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let line = line.strip_prefix("Game ").unwrap();
        let (id, rest) = line.split_once(": ").unwrap();
        let id = id.parse().unwrap();
        let sets = rest
            .split("; ")
            .map(|set| set.split(", ").map(CubesEntry::from).collect())
            .collect();

        Game { id, sets }
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().flatten().all(CubesEntry::is_valid)
    }

    fn calc_power(&self) -> usize {
        let mut max_green = 0;
        let mut max_red = 0;
        let mut max_blue = 0;

        self.sets
            .iter()
            .flatten()
            .for_each(|entry| match entry.color {
                Color::Blue => max_blue = max_blue.max(entry.count),
                Color::Red => max_red = max_red.max(entry.count),
                Color::Green => max_green = max_green.max(entry.count),
            });

        max_green * max_red * max_blue
    }
}

fn calc_sum_valid(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .filter(Game::is_valid)
        .map(|game| game.id)
        .sum()
}

fn calc_min_power(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .map(|game| game.calc_power())
        .sum()
}

fn part_1(input: &str) {
    let answer_1 = calc_sum_valid(input);

    println!("Part 1 answer is {answer_1}");
}

fn part_2(input: &str) {
    let answer_2 = calc_min_power(input);

    println!("Part 2 answer is {answer_2}");
}

pub fn run() {
    let input = read_text_from_file("23", "02");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_solution() {
        assert_eq!(calc_sum_valid(INPUT), 8);
        assert_eq!(calc_min_power(INPUT), 2286);
    }
}

