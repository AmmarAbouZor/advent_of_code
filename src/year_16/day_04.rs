use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::utls::{get_input_path, read_lines_from_file};

#[derive(Debug)]
struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (parts, checksum) = s.split_once('[').unwrap();

        let (name, id) = parts.rsplit_once('-').unwrap();
        let id = id.parse()?;
        let name = name.to_owned();

        let checksum = checksum.strip_suffix(']').unwrap().to_owned();

        Ok(Room { name, id, checksum })
    }
}

static ALPHA_CHARS_COUNT: u32 = 26;
impl Room {
    fn is_valid(&self) -> bool {
        let mut chars: Vec<char> = self.name.chars().filter(|ch| *ch != '-').collect();
        let chars_clone = chars.clone();

        chars.sort_by(|a, b| {
            let count_a = chars_clone.iter().filter(|&c| c == a).count();
            let count_b = chars_clone.iter().filter(|&c| c == b).count();
            let mut result = count_b.partial_cmp(&count_a).unwrap();
            if result == Ordering::Equal {
                result = a.partial_cmp(&b).unwrap();
            }

            result
        });

        let chars_checksum: String = chars.into_iter().unique().take(5).collect();

        chars_checksum == self.checksum
    }

    fn decrypt(&self) -> String {
        let decrypt_char = |ch: char, cycles: u32| {
            let mut value = ch as u32 + cycles;
            if value > b'z' as u32 {
                value -= ALPHA_CHARS_COUNT;
            }

            char::from_u32(value).unwrap()
        };

        let cycles = self.id % ALPHA_CHARS_COUNT;
        self.name
            .chars()
            .map(|ch| match ch {
                '-' => ' ',
                ch => decrypt_char(ch, cycles),
            })
            .collect()
    }
}

fn get_lines() -> Vec<String> {
    read_lines_from_file(&get_input_path("16", "04"))
}

fn part_1() {
    let sum: u32 = get_lines()
        .iter()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_valid())
        .map(|room| room.id)
        .sum();

    println!("sum of real rooms is {sum}");
}

fn part_2() {
    let id = get_lines()
        .iter()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_valid())
        .find(|room| room.decrypt() == String::from("northpole object storage"))
        .unwrap()
        .id;

    println!("Sector ID is {id}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_room() {
        assert!("aaaaa-bbb-z-y-x-123[abxyz]"
            .parse::<Room>()
            .unwrap()
            .is_valid());
        assert!("a-b-c-d-e-f-g-h-987[abcde]"
            .parse::<Room>()
            .unwrap()
            .is_valid());
        assert!("not-a-real-room-404[oarel]"
            .parse::<Room>()
            .unwrap()
            .is_valid());
        assert!(!"totally-real-room-200[decoy]"
            .parse::<Room>()
            .unwrap()
            .is_valid());
    }

    #[test]
    fn test_decrypt() {
        let room: Room = "qzmt-zixmtkozy-ivhz-343[abxyz]".parse().unwrap();

        assert_eq!(room.decrypt(), String::from("very encrypted name"));
    }
}
