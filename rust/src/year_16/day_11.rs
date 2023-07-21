use crate::utls::read_text_from_file;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum ItemType {
    Generator,
    Microchip,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    floor: isize,
    item_type: ItemType,
}

impl Item {
    fn new(name: String, floor: isize, item_type: ItemType) -> Self {
        Self {
            name,
            floor,
            item_type,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn to_int(&self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
        }
    }

    fn rand(rng: &mut ThreadRng) -> Direction {
        if rng.gen::<bool>() {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    fn rand_ratio(rng: &mut ThreadRng, ratio: f64) -> Direction {
        if rng.gen_bool(ratio) {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

#[derive(Clone)]
struct Move {
    items: [Option<(String, ItemType)>; 2],
    dir: Direction,
}

struct Game {
    items: Vec<Item>,
    cur_floor: isize,
}

impl Game {
    fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            cur_floor: 1,
        }
    }

    fn apply_move(&mut self, mov: Move) {
        let new_floor = self.cur_floor + mov.dir.to_int();
        assert!(new_floor > 0);
        assert!(new_floor <= 4);

        let items: Vec<_> = mov.items.into_iter().flatten().collect();
        assert!(!items.is_empty());
        assert!(items.len() <= 2);

        items.into_iter().for_each(|(name, typ)| {
            let item = self
                .items
                .iter_mut()
                .find(|item| item.name == name && item.item_type == typ)
                .unwrap();
            assert!(item.floor == self.cur_floor);
            item.floor = new_floor;
        });

        self.cur_floor = new_floor;
    }

    fn success(&self) -> bool {
        self.items.iter().all(|item| item.floor == 4)
    }

    fn has_lost(&self) -> bool {
        for chip in self
            .items
            .iter()
            .filter(|t| t.item_type == ItemType::Microchip)
        {
            let match_gen = self
                .items
                .iter()
                .find(|t| t.item_type == ItemType::Generator && t.name == chip.name)
                .unwrap();
            if match_gen.floor != chip.floor
                && self
                    .items
                    .iter()
                    .filter(|item| item.item_type == ItemType::Generator)
                    .any(|gen| gen.floor == chip.floor)
            {
                return true;
            }
        }

        false
    }

    fn create_rand_move(&self, rng: &mut ThreadRng) -> Option<Move> {
        let items: Vec<_> = self
            .items
            .iter()
            .filter(|item| item.floor == self.cur_floor)
            .map(|item| (item.name.clone(), item.item_type))
            .collect();

        let mut gens = vec![];
        let micros: Vec<_> = items
            .iter()
            .filter(|(name, typ)| {
                if *typ == ItemType::Microchip {
                    true
                } else {
                    gens.push((name.to_owned(), *typ));
                    false
                }
            })
            .cloned()
            .collect();

        let mut counter = 0;

        while counter < 15 {
            let mut cloned_items = items.clone();
            cloned_items.shuffle(rng);

            //optimize direction
            let dir = if self.items.iter().all(|item| item.floor >= self.cur_floor) {
                Direction::Up
            } else {
                let mut dir = if counter < 1 {
                    let cur_count = items.len();
                    let prev_count = self
                        .items
                        .iter()
                        .filter(|i| i.floor == self.cur_floor - 1)
                        .count();
                    if prev_count > cur_count {
                        Direction::rand_ratio(rng, 0.1)
                    } else {
                        let next_count = self
                            .items
                            .iter()
                            .filter(|i| i.floor == self.cur_floor + 1)
                            .count();
                        if next_count < cur_count {
                            Direction::rand_ratio(rng, 0.9)
                        } else {
                            Direction::rand(rng)
                        }
                    }
                } else {
                    Direction::rand(rng)
                };

                let next_floor = self.cur_floor + dir.to_int();
                if !next_floor.is_positive() || next_floor > 4 {
                    dir = match dir {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                    };
                }

                dir
            };

            //optimize items;
            let final_items = if dir == Direction::Up {
                match counter {
                    c if c < 2 && micros.len() > 1 => {
                        let mut cloned = micros.clone();
                        cloned.shuffle(rng);

                        [cloned.pop(), cloned.pop()]
                    }
                    c if c < 4 && gens.len() > 1 => {
                        let mut cloned = gens.clone();
                        cloned.shuffle(rng);

                        [cloned.pop(), cloned.pop()]
                    }
                    c if c < 8 => [cloned_items.pop(), cloned_items.pop()],
                    _ => [cloned_items.pop(), None],
                }
            } else if counter < 8 {
                [cloned_items.pop(), None]
            } else {
                [cloned_items.pop(), cloned_items.pop()]
            };

            let mv = Move {
                items: final_items,
                dir,
            };

            if !self.check_wrong_mv(mv.clone()) {
                return Some(mv);
            }

            counter += 1;
        }

        None
    }

    fn check_wrong_mv(&self, mv: Move) -> bool {
        let next_floor = self.cur_floor + mv.dir.to_int();
        let mut pot_items: Vec<_> = self
            .items
            .iter()
            .filter(|item| item.floor == self.cur_floor || item.floor == next_floor)
            .cloned()
            .collect();

        mv.items.into_iter().flatten().for_each(|(name, typ)| {
            let item = pot_items
                .iter_mut()
                .find(|item| item.name == name && item.item_type == typ)
                .unwrap();
            item.floor = next_floor;
        });

        for chip in pot_items
            .iter()
            .filter(|t| t.item_type == ItemType::Microchip)
        {
            let match_gen = pot_items
                .iter()
                .find(|t| t.item_type == ItemType::Generator && t.name == chip.name);

            if (match_gen.is_none() || match_gen.unwrap().floor != chip.floor)
                && pot_items
                    .iter()
                    .filter(|item| item.item_type == ItemType::Generator)
                    .any(|gen| gen.floor == chip.floor)
            {
                return true;
            }
        }

        false
    }
}

fn fetch_input() -> Vec<Item> {
    let mut items = vec![];

    for (i, line) in read_text_from_file("16", "11").lines().take(3).enumerate() {
        for part in line.split(',') {
            let mut words = part.split_whitespace();
            let item_type = match words.next_back().unwrap() {
                word if word.starts_with("gener") => ItemType::Generator,
                _ => ItemType::Microchip,
            };

            let mut name = words.next_back().unwrap();
            if item_type == ItemType::Microchip {
                name = name.split('-').next().unwrap();
            }

            items.push(Item::new(name.to_owned(), i as isize + 1, item_type));
        }
    }

    items
}

fn part_1() {
    let items = fetch_input();

    let mut min_move = usize::MAX;

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let mut game = Game::new(items.clone());
        let mut count = 0;
        while count < min_move && !game.has_lost() {
            if let Some(mov) = game.create_rand_move(&mut rng) {
                game.apply_move(mov);
                count += 1;
                if game.success() {
                    println!("success {count}");
                    min_move = min_move.min(count);
                    break;
                }
            } else {
                break;
            }
        }
    }

    println!("part 1: min is {min_move}");
}

fn part_2() {
    let mut items = fetch_input();

    items.push(Item::new("elerium".into(), 1, ItemType::Generator));
    items.push(Item::new("elerium".into(), 1, ItemType::Microchip));
    items.push(Item::new("dilithium".into(), 1, ItemType::Generator));
    items.push(Item::new("dilithium".into(), 1, ItemType::Microchip));

    let mut min_move = usize::MAX;

    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let mut game = Game::new(items.clone());
        let mut count = 0;
        while count < min_move && !game.has_lost() {
            if let Some(mov) = game.create_rand_move(&mut rng) {
                game.apply_move(mov);
                count += 1;
                if game.success() {
                    println!("success {count}");
                    min_move = min_move.min(count);
                    break;
                }
            } else {
                break;
            }
        }
    }

    println!("part 2: min is {min_move}");
}

pub fn run() {
    part_1();
    part_2();
}
