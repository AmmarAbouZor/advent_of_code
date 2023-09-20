use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Player {
    id: usize,
    score: usize,
    position: usize,
}

impl From<&str> for Player {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();

        let id: usize = parts.nth(1).and_then(|id| id.parse().ok()).unwrap();

        let position = parts.last().and_then(|pos| pos.parse().ok()).unwrap();

        Player {
            id,
            score: 0,
            position,
        }
    }
}

impl Player {
    fn play_round(&mut self, turns: [usize; 3]) -> bool {
        self.position += turns.into_iter().sum::<usize>();
        while self.position > 10 {
            self.position -= 10;
        }
        self.position = self.position;
        self.score += self.position;

        self.score >= 1000
    }
}

#[derive(Debug, Default)]
struct DetermDice {
    current_side: usize,
    counter: usize,
}

impl DetermDice {
    fn next_three(&mut self) -> [usize; 3] {
        self.counter += 3;

        let mut result = [3; 3];
        for i in 0..3 {
            self.current_side += 1;
            if self.current_side > 100 {
                self.current_side -= 100;
            }
            result[i] = self.current_side;
        }

        result
    }
}

fn parse_input(input: &str) -> [Player; 2] {
    let mut lines = input.lines();
    let player_1 = lines.next().map(Player::from).unwrap();
    let player_2 = lines.next().map(Player::from).unwrap();

    [player_1, player_2]
}

fn calc_loser_score(input: &str) -> usize {
    let mut players = parse_input(input);
    let mut dice = DetermDice::default();

    loop {
        for player in players.iter_mut() {
            let sides = dice.next_three();
            if player.play_round(sides) {
                return players.into_iter().map(|p| p.score).min().unwrap() * dice.counter;
            }
        }
    }
}

fn part_1() {
    let input = read_text_from_file("21", "21");
    let answer = calc_loser_score(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_loser_score(INPUT), 739785);
    }
}

