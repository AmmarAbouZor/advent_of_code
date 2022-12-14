use std::collections::HashSet;

use rand::{rngs::ThreadRng, seq::SliceRandom};

const FAV_NUM: i32 = 1362;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_wall(&self, seed: i32) -> bool {
        let num =
            self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y + seed;

        num.count_ones() & 1 != 0
    }
}

struct Game {
    cur_pos: Pos,
    fav_num: i32,
    pos_cache: HashSet<Pos>,
    last_pos: Pos,
}

impl Game {
    fn start(fav_num: i32) -> Game {
        let start_pos = Pos::new(1, 1);
        Game {
            cur_pos: start_pos,
            fav_num,
            pos_cache: HashSet::from([start_pos]),
            last_pos: start_pos,
        }
    }

    fn is_wall(&self, pos: &Pos) -> bool {
        Pos::new(pos.x, pos.y).is_wall(self.fav_num)
    }

    fn is_pos_valid(&self, pos: &Pos) -> bool {
        pos.x > -1
            && pos.y > -1
            && *pos != self.last_pos
            && !self.is_wall(pos)
            && !self.pos_cache.contains(pos)
    }

    fn next_possible_positions(&self) -> Vec<Pos> {
        let mut positions = vec![];
        let next_x = Pos::new(self.cur_pos.x + 1, self.cur_pos.y);
        if self.is_pos_valid(&next_x) {
            positions.push(next_x);
        }
        let next_y = Pos::new(self.cur_pos.x, self.cur_pos.y + 1);
        if self.is_pos_valid(&next_y) {
            positions.push(next_y);
        }

        let prev_x = Pos::new(self.cur_pos.x - 1, self.cur_pos.y);
        if self.is_pos_valid(&prev_x) {
            positions.push(prev_x);
        }
        let prev_y = Pos::new(self.cur_pos.x, self.cur_pos.y - 1);
        if self.is_pos_valid(&prev_y) {
            positions.push(prev_y);
        }

        positions
    }

    fn move_to(&mut self, pos: Pos) {
        self.last_pos = self.cur_pos;
        self.cur_pos = pos;
        self.pos_cache.insert(pos);
    }

    fn play(&mut self, target: Pos, min_moves: usize, rng: &mut ThreadRng) -> Option<usize> {
        let in_target_direction = |pos: &Pos, cur: &Pos| {
            (cur.x - target.x * cur.x - pos.x).is_positive()
                || (cur.y - target.y * cur.y - pos.y).is_positive()
        };

        while self.pos_cache.len() <= min_moves {
            let possible_pos = self.next_possible_positions();
            if possible_pos.is_empty() {
                break;
            }

            let mut next_pos = possible_pos.choose(rng).unwrap().to_owned();
            if possible_pos.len() > 1 && !in_target_direction(&next_pos, &self.cur_pos) {
                next_pos = possible_pos.choose(rng).unwrap().to_owned();
            }

            self.move_to(next_pos);

            if self.cur_pos == target {
                return Some(self.pos_cache.len() - 1);
            }
        }

        None
    }

    fn get_locations(mut self, steps: usize, rng: &mut ThreadRng) -> HashSet<Pos> {
        for _ in 0..steps {
            let possible_pos = self.next_possible_positions();
            if possible_pos.is_empty() {
                break;
            }

            let next_pos = *possible_pos.choose(rng).unwrap();

            self.move_to(next_pos);
        }

        self.pos_cache
    }
}

fn get_best_route(target: Pos, seed: i32) -> usize {
    let mut min_moves = usize::MAX;

    let mut rng = rand::thread_rng();

    for _ in 0..1000000000 {
        let mut game = Game::start(seed);
        if let Some(moves) = game.play(target, min_moves, &mut rng) {
            if min_moves == moves {
                break;
            }
            min_moves = min_moves.min(moves);
        }
    }

    min_moves
}

fn part_1() {
    let best_route = get_best_route(Pos::new(31, 39), FAV_NUM);

    println!("part_1; best route is {best_route}");
}

fn part_2() {
    let mut rng = rand::thread_rng();

    let mut all_locations = HashSet::new();
    for _ in 0..1000000 {
        let locations = Game::start(FAV_NUM).get_locations(50, &mut rng);
        all_locations.extend(locations);
    }

    println!("locations are {}", all_locations.len());
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_wall() {
        let game = Game::start(10);
        assert!(!game.is_wall(&Pos::new(0, 0)));
        assert!(game.is_wall(&Pos::new(1, 0)));
        assert!(!game.is_wall(&Pos::new(0, 1)));
        assert!(game.is_wall(&Pos::new(6, 2)));
        assert!(game.is_wall(&Pos::new(9, 6)));
    }

    #[test]
    fn test_game() {
        assert_eq!(get_best_route(Pos::new(7, 4), 10), 11);
    }
}
