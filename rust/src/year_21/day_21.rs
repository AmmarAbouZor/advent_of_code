use std::collections::HashMap;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl From<&str> for Player {
    fn from(value: &str) -> Self {
        let (_, pos) = value.split_once(": ").unwrap();

        let position = pos.parse().unwrap();

        Player { score: 0, position }
    }
}

impl Player {
    fn play_round_triple(&mut self, turns: [usize; 3]) -> bool {
        self.position += turns.into_iter().sum::<usize>();
        while self.position > 10 {
            self.position -= 10;
        }
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Universe {
    pos_1: usize,
    pos_2: usize,
    score_1: usize,
    score_2: usize,
}

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
        let mut positions = input.lines().map(|line| {
            let (_, pos) = line.split_once(": ").unwrap();
            pos.parse().unwrap()
        });

        let pos_1 = positions.next().unwrap();
        let pos_2 = positions.next().unwrap();
        let score_1 = 0;
        let score_2 = 0;

        Universe {
            pos_1,
            pos_2,
            score_1,
            score_2,
        }
    }
}

impl Universe {
    fn new(pos_1: usize, pos_2: usize, score_1: usize, score_2: usize) -> Self {
        Self {
            pos_1,
            pos_2,
            score_1,
            score_2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Score {
    p1: usize,
    p2: usize,
}

impl Score {
    fn new(p1: usize, p2: usize) -> Self {
        Self { p1, p2 }
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
            if player.play_round_triple(sides) {
                return players.into_iter().map(|p| p.score).min().unwrap() * dice.counter;
            }
        }
    }
}

fn calc_universes(input: &str) -> usize {
    let init_universe = Universe::from(input);
    let mut cache = HashMap::new();

    let final_score = calc_uni_rec(init_universe, &mut cache);

    final_score.p1.max(final_score.p2)
}

fn calc_uni_rec(universe: Universe, cache: &mut HashMap<Universe, Score>) -> Score {
    if universe.score_1 >= 21 {
        return Score::new(1, 0);
    }
    if universe.score_2 >= 21 {
        return Score::new(0, 1);
    }

    if let Some(score) = cache.get(&universe) {
        return *score;
    }

    let mut score = Score::default();

    for die_1 in 1..4 {
        for die_2 in 1..4 {
            for die_3 in 1..4 {
                let mut pos_1 = universe.pos_1 + die_1 + die_2 + die_3;
                while pos_1 > 10 {
                    pos_1 -= 10;
                }
                let score_1 = universe.score_1 + pos_1;
                // Swap players and scores since it's next player turn
                let new_universe = Universe::new(universe.pos_2, pos_1, universe.score_2, score_1);
                let rec_score = calc_uni_rec(new_universe, cache);
                score.p1 += rec_score.p2;
                score.p2 += rec_score.p1;
            }
        }
    }

    assert!(cache.insert(universe, score).is_none());
    score
}

fn part_1() {
    let input = read_text_from_file("21", "21");
    let answer = calc_loser_score(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "21");
    let answer = calc_universes(input.as_str());

    println!("Part 2 answer is {answer}");
}

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
    fn test_whole() {
        assert_eq!(calc_loser_score(INPUT), 739785);
        assert_eq!(calc_universes(INPUT), 444356092776315);
    }
}
