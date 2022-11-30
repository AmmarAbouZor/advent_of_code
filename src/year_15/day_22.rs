#![allow(warnings, unused)]

use rand::{rngs::ThreadRng, seq::SliceRandom};

#[derive(Debug, Clone)]
struct Boss {
    hit_points: i16,
    damage: i8,
}

impl Boss {
    fn new(hit_points: i16, damage: i8) -> Self {
        Self { hit_points, damage }
    }

    fn create_from_input() -> Self {
        Self::new(55, 8)
    }
}

#[derive(Debug, Clone)]
struct Wizard {
    hit_points: i16,
    mana: i16,
    spent_mana: i16,
    magic_missile_count: i8,
    drain_count: i8,
    shield_count: i8,
    poison_count: i8,
    recharge_count: i8,
    damage: Option<i8>,
    armor: Option<i8>,
}

impl Wizard {
    const MAGIC_MISSILE_COST: i16 = 53;
    const DRAIN_COST: i16 = 73;
    const SHIELD_COST: i16 = 113;
    const POISON_COST: i16 = 173;
    const RECHARGE_COST: i16 = 229;

    fn new(hit_points: i16, mana: i16) -> Self {
        Self {
            hit_points,
            mana,
            spent_mana: 0,
            magic_missile_count: 0,
            drain_count: 0,
            shield_count: 0,
            poison_count: 0,
            recharge_count: 0,
            damage: None,
            armor: None,
        }
    }

    fn create_from_input() -> Self {
        Self::new(50, 500)
    }

    fn purchase_cast(&mut self, cost: i16) {
        assert!(self.mana >= cost);
        self.mana -= cost;
        self.spent_mana += cost;
    }

    fn cast_magic_missile(&mut self) {
        self.purchase_cast(Wizard::MAGIC_MISSILE_COST);
        self.magic_missile_count = 1;
        self.apply_turn();
    }

    fn cast_drain(&mut self) {
        self.purchase_cast(Wizard::DRAIN_COST);
        self.drain_count = 1;
        self.apply_turn();
    }

    fn cast_shield(&mut self) {
        self.apply_turn();
        self.purchase_cast(Wizard::SHIELD_COST);
        self.shield_count = 6;
    }

    fn cast_poison(&mut self) {
        self.apply_turn();
        self.purchase_cast(Wizard::POISON_COST);
        self.poison_count = 6;
    }

    fn cast_recharge(&mut self) {
        self.apply_turn();
        self.purchase_cast(Wizard::RECHARGE_COST);
        self.recharge_count = 5;
    }

    fn apply_turn(&mut self) {
        let mut damage = 0;
        let mut armor = 0;
        if self.magic_missile_count.is_positive() {
            self.magic_missile_count = 0;
            damage += 4;
        } else if self.drain_count.is_positive() {
            self.drain_count = 0;
            damage += 2;
            self.hit_points += 2;
        }

        if self.poison_count.is_positive() {
            self.poison_count -= 1;
            damage += 3;
        };

        if self.shield_count.is_positive() {
            self.shield_count -= 1;
            armor += 7;
        }

        if self.recharge_count.is_positive() {
            self.recharge_count -= 1;
            self.mana += 101;
        }

        self.damage = Some(damage);
        self.armor = Some(armor);
    }

    fn consume_damage(&mut self) -> i8 {
        self.damage.take().unwrap()
    }

    fn consume_armor(&mut self) -> i8 {
        self.armor.take().unwrap()
    }
}

struct Game {
    wizard: Wizard,
    boss: Boss,
}

impl Game {
    fn new(wizard: Wizard, boss: Boss) -> Self {
        Self { wizard, boss }
    }

    fn print_status(&self) {
        println!(
            "boss:\n    hit_points: {}\n    damage: {}",
            self.boss.hit_points, self.boss.damage
        );

        println!(
            "wizard:\n    hit_points: {}\n    mana: {}\n    shield: {}\n    poison: {}\n    recharge: {}",
             self.wizard.hit_points, self.wizard.mana, self.wizard.shield_count,
             self.wizard.poison_count, self.wizard.recharge_count);
    }

    fn print_key_bindings() {
        println!(
            "keys: m: magic missile | d: drain | s: shield | p: poison | r: recharge | enter: pass"
        );
    }

    fn still_playing(&self) -> bool {
        self.wizard.hit_points.is_positive() && self.boss.hit_points.is_positive()
    }

    fn is_wizard_winner(&self) -> bool {
        assert!(!self.still_playing());

        self.wizard.hit_points.is_positive()
    }

    fn apply_input(&mut self, input: &str) -> bool {
        let mut valid_input = true;

        match input {
            "m" => self.wizard.cast_magic_missile(),
            "d" => self.wizard.cast_drain(),
            "s" => self.wizard.cast_shield(),
            "p" => self.wizard.cast_poison(),
            "r" => self.wizard.cast_recharge(),
            "" => self.wizard.apply_turn(),
            _ => valid_input = false,
        }

        valid_input
    }

    fn play_wizard_turn(&mut self, input: &str) {
        self.apply_input(input);
        let wizard_damage = self.wizard.consume_damage();
        self.boss.hit_points -= wizard_damage as i16;
    }

    fn play_boss_turn(&mut self) {
        self.wizard.apply_turn();
        let wizard_damage = self.wizard.consume_damage();
        let wizard_armor = self.wizard.consume_armor();

        self.boss.hit_points -= wizard_damage as i16;

        if self.boss.hit_points.is_positive() {
            let boss_total_damage = match self.boss.damage - wizard_armor {
                damage if damage.is_positive() => damage as i16,
                _ => 1,
            };

            self.wizard.hit_points -= boss_total_damage;
        }
    }

    fn get_valid_input(&self, rng: &mut ThreadRng) -> String {
        let mut found = false;
        let mut input = "";

        let choices = ["m", "d", "s", "p", "r", ""];
        while !found {
            input = choices.choose(rng).unwrap();
            found = self.is_input_valid(input);
        }

        input.into()
    }

    fn is_input_valid(&self, input: &str) -> bool {
        match input {
            "m" => Wizard::MAGIC_MISSILE_COST <= self.wizard.mana,
            "d" => Wizard::DRAIN_COST <= self.wizard.mana,
            "s" => {
                Wizard::SHIELD_COST <= self.wizard.mana && !self.wizard.shield_count.is_positive()
            }
            "p" => {
                Wizard::POISON_COST <= self.wizard.mana && !self.wizard.poison_count.is_positive()
            }
            "r" => {
                Wizard::RECHARGE_COST <= self.wizard.mana
                    && !self.wizard.recharge_count.is_positive()
            }
            "" => true,
            _ => panic!("invalid input"),
        }
    }

    fn simulate_battle(&mut self, rng: &mut ThreadRng) -> bool {
        while self.still_playing() {
            let input = self.get_valid_input(rng);
            self.play_wizard_turn(&input);

            if !self.still_playing() {
                break;
            }

            self.play_boss_turn();
        }

        self.is_wizard_winner()
    }

    fn simulate_battle_hard(&mut self, rng: &mut ThreadRng) -> bool {
        while self.still_playing() {
            self.wizard.hit_points -= 1;
            if !self.wizard.hit_points.is_positive() {
                return false;
            }

            let input = self.get_valid_input(rng);
            self.play_wizard_turn(&input);

            if !self.still_playing() {
                break;
            }

            self.wizard.hit_points -= 1;
            if !self.wizard.hit_points.is_positive() {
                return false;
            }

            self.play_boss_turn();
        }

        self.is_wizard_winner()
    }
}

fn part_1() {
    let mut min_mana = i16::MAX;
    let mut rng = rand::thread_rng();
    //try 1000_000 times
    for _ in 0..1000_000 {
        let mut game = Game::new(Wizard::create_from_input(), Boss::create_from_input());
        if game.simulate_battle(&mut rng) {
            min_mana = min_mana.min(game.wizard.spent_mana);
        }
    }

    println!("after million simulation: {min_mana}");
}

fn part_2() {
    let mut min_mana = i16::MAX;
    let mut rng = rand::thread_rng();
    //try 4000_000 times
    for _ in 0..4000_000 {
        let mut game = Game::new(Wizard::create_from_input(), Boss::create_from_input());
        if game.simulate_battle_hard(&mut rng) {
            min_mana = min_mana.min(game.wizard.spent_mana);
        }
    }

    println!("after four millions simulation on hard: {min_mana}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_battle() {
        let mut game = Game::new(Wizard::new(10, 250), Boss::new(13, 8));

        game.play_wizard_turn("p");
        game.play_boss_turn();

        assert!(game.still_playing());

        game.play_wizard_turn("m");
        game.play_boss_turn();

        assert!(!game.still_playing());
        assert!(game.is_wizard_winner());
    }

    #[test]
    fn test_more_complicated_battle() {
        let mut game = Game::new(Wizard::new(10, 250), Boss::new(14, 8));

        game.play_wizard_turn("r");
        game.play_boss_turn();

        assert!(game.still_playing());

        game.play_wizard_turn("s");
        game.play_boss_turn();

        assert!(game.still_playing());

        game.play_wizard_turn("d");
        game.play_boss_turn();

        assert!(game.still_playing());

        game.play_wizard_turn("p");
        game.play_boss_turn();

        assert!(game.still_playing());

        game.play_wizard_turn("m");
        game.play_boss_turn();

        assert!(!game.still_playing());
        assert!(game.is_wizard_winner());
    }
}
