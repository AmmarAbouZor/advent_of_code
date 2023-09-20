use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl From<&str> for Player {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace();

        let position = parts.last().and_then(|pos| pos.parse().ok()).unwrap();

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

    fn move_position(&mut self, dice: usize) {
        self.position += dice;
        while self.position > 10 {
            self.position -= 10;
        }
    }

    fn add_score(&mut self) -> bool {
        self.score += self.position;

        self.score >= 21
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

#[derive(Debug, Clone)]
struct Universe {
    players: [Player; 2],
    turn_stack: Vec<usize>,
}

impl Universe {
    fn new(players: [Player; 2], turn_stack: Vec<usize>) -> Self {
        Self {
            players,
            turn_stack,
        }
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
    let mut scores_map = [0; 2];

    let players = parse_input(input);

    let universe = Universe::new(players, vec![0; 3]);

    let mut uni_stack = vec![universe];

    while let Some(mut universe) = uni_stack.pop() {
        let turn_idx = universe.turn_stack.pop().unwrap();
        let end_turn = universe.turn_stack.is_empty();
        if end_turn {
            let next_idx = (turn_idx + 1) % 2;
            universe.turn_stack.extend([next_idx; 3].into_iter());
        }
        for dice in 1..4 {
            universe.players[turn_idx].move_position(dice);
            if end_turn && universe.players[turn_idx].add_score() {
                scores_map[turn_idx] += 1;
                println!("{:?}", scores_map);
            } else {
                uni_stack.push(universe.clone());
            }
        }
    }

    scores_map.into_iter().max().unwrap()
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

