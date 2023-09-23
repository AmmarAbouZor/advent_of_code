use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    corridor: [u8; 11],
    rooms: [[u8; 2]; 4],
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let corridore = [b'.'; 11];
        let lines: Vec<_> = value
            .lines()
            .skip(2)
            .take(2)
            .map(|line| line.as_bytes())
            .collect();
        let rooms = [
            [lines[0][3], lines[1][3]],
            [lines[0][5], lines[1][5]],
            [lines[0][7], lines[1][7]],
            [lines[0][9], lines[1][9]],
        ];

        State {
            corridor: corridore,
            rooms,
        }
    }
}

impl State {
    fn print(&self) {
        println!(
            "{}",
            self.corridor.iter().map(|b| *b as char).collect::<String>()
        );

        for i in 0..2 {
            println!(
                "  {} {} {} {}  ",
                self.rooms[0][i] as char,
                self.rooms[1][i] as char,
                self.rooms[2][i] as char,
                self.rooms[3][i] as char
            );
        }
    }

    fn is_solved(&self) -> bool {
        self.rooms
            .iter()
            .zip("ABCD".bytes())
            .all(|(room, ch)| room.iter().all(|b| *b == ch))
    }

    /// this represent the ids in the corridor directly above the rooms
    const CORR_ROOMS_IDX: [usize; 4] = [2, 4, 6, 8];

    fn get_all_valid_moves(&self) -> Vec<(usize, State)> {
        let rooms_depth = self.rooms[0].len();

        let mut moves = Vec::new();
        // === Moving parts from corridor to rooms ===
        for corr_idx in 0..self.corridor.len() {
            if self.corridor[corr_idx] == b'.' {
                continue;
            }

            let target_room_idx = (self.corridor[corr_idx] - b'A') as usize;
            // Check if the way in the corridor to the target room is free
            let room_corr_idx = State::CORR_ROOMS_IDX[target_room_idx];
            let (corr_start, corr_end) = if corr_idx > room_corr_idx {
                (room_corr_idx, corr_idx)
            } else {
                (corr_idx + 1, room_corr_idx + 1)
            };
            if (corr_start..corr_end).any(|i| self.corridor[i] != b'.') {
                continue;
            }

            // find the last free slot in the room
            let Some(free_room_idx) = (0..rooms_depth)
                .take_while(|i| self.rooms[target_room_idx][*i] == b'.')
                .last()
            else {
                continue;
            };
            // Check is the room doesn't have any wrong items
            if (free_room_idx + 1..rooms_depth)
                .any(|i| self.rooms[target_room_idx][i] != self.corridor[corr_idx])
            {
                continue;
            }

            moves.push(self.make_move(corr_idx, target_room_idx, free_room_idx));
        }

        // === Moving Parts out of the rooms ===
        for room_idx in 0..4 {
            let Some(non_free_idx) = (0..rooms_depth).find(|i| self.rooms[room_idx][*i] != b'.')
            else {
                continue;
            };

            let room_corr_idx = State::CORR_ROOMS_IDX[room_idx];
            // Right possibilities
            let moves_right = (room_corr_idx..self.corridor.len())
                .take_while(|i| self.corridor[*i] == b'.')
                .filter(|i| !State::CORR_ROOMS_IDX.contains(i))
                .map(|i| self.make_move(i, room_idx, non_free_idx));
            moves.extend(moves_right);

            // Left possibilities
            let moves_left = (0..room_corr_idx)
                .rev()
                .take_while(|i| self.corridor[*i] == b'.')
                .filter(|i| !Self::CORR_ROOMS_IDX.contains(i))
                .map(|i| self.make_move(i, room_idx, non_free_idx));
            moves.extend(moves_left);
        }

        moves
    }

    fn make_move(&self, idx: usize, room_idx: usize, room_depth: usize) -> (usize, State) {
        let amph = if self.corridor[idx] == b'.' {
            self.rooms[room_idx][room_depth] as char
        } else {
            self.corridor[idx] as char
        };

        let room_corr_idx = State::CORR_ROOMS_IDX[room_idx];
        let moves = room_depth + idx.abs_diff(room_corr_idx) + 1;
        let cost = moves * Self::get_amph_cost(amph);
        let mut clone = self.clone();

        std::mem::swap(
            &mut clone.corridor[idx],
            &mut clone.rooms[room_idx][room_depth],
        );

        (cost, clone)
    }

    fn get_amph_cost(amph: char) -> usize {
        match amph {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }
}

fn calc_least_energy(input: &str) -> usize {
    let state = State::from(input);
    state.print();

    let mut energy_map: HashMap<State, usize> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), state));
    while let Some((Reverse(cost), state)) = queue.pop() {
        if state.is_solved() {
            return cost;
        }

        if energy_map.get(&state).is_some_and(|c| cost > *c) {
            continue;
        }

        for (new_cost, new_state) in state.get_all_valid_moves() {
            let next_cost = cost + new_cost;
            let saved_cost = *energy_map.get(&new_state).unwrap_or(&usize::MAX);
            if next_cost < saved_cost {
                energy_map.insert(new_state.clone(), next_cost);
                queue.push((Reverse(next_cost), new_state));
            }
        }
    }

    unreachable!()
}

fn part_1() {
    let input = read_text_from_file("21", "23");
    let answer = calc_least_energy(input.as_str());

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

    const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_least_energy(INPUT), 12521)
    }
}

