use crate::utls;
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Left(i16),
    Right(i16),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        match &input[..1] {
            "L" => Instruction::Left(input[1..].parse().unwrap()),
            "R" => Instruction::Right(input[1..].parse().unwrap()),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Default)]
struct Player {
    x: i16,
    y: i16,
    last_dir: u8,
}

impl Player {
    fn apply_ins(&mut self, instr: Instruction) {
        let (dir, step) = match instr {
            Instruction::Left(step) => ((self.last_dir + 4 - 1) % 4, step),
            Instruction::Right(step) => ((self.last_dir + 1) % 4, step),
        };

        match dir {
            0 => self.y += step,
            1 => self.x += step,
            2 => self.y -= step,
            3 => self.x -= step,
            _ => panic!("dir out of range"),
        }

        self.last_dir = dir;
    }

    fn get_distance(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}

fn part_1() {
    let mut player = Player::default();
    let input = utls::read_text_from_file("16", "01");
    input.split(", ").map(Instruction::from).for_each(|ins| {
        player.apply_ins(ins);
    });

    println!("distance to target is {}", player.get_distance());
}

fn get_distance_first_visited_block() -> Option<i16> {
    let mut player = Player::default();
    let input = utls::read_text_from_file("16", "01");
    let mut positions = HashSet::new();
    for ins in input.split(", ").map(Instruction::from) {
        let (last_x, last_y) = (player.x, player.y);

        player.apply_ins(ins);

        for x in last_x.min(player.x)..=last_x.max(player.x) {
            for y in last_y.min(player.y)..=last_y.max(player.y) {
                if (x, y) == (last_x, last_y) {
                    continue;
                }

                if !positions.insert((x, y)) {
                    return Some(x.abs() + y.abs());
                }
            }
        }
    }

    None
}

fn part_2() {
    let distance = get_distance_first_visited_block().unwrap();
    println!("distance to first visited block is {}", distance);
}

pub fn run() {
    part_1();
    part_2();
}
