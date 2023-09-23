use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State<const N: usize> {
    corridor: [u8; 11],
    rooms: [[u8; N]; 4],
}

impl From<&str> for State<2> {
    fn from(value: &str) -> Self {
        let corridor = [b'.'; 11];
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

        State { corridor, rooms }
    }
}

// This has the added values after unfolding
impl From<&str> for State<4> {
    fn from(value: &str) -> Self {
        let corridor = [b'.'; 11];
        let lines: Vec<_> = value
            .lines()
            .skip(2)
            .take(2)
            .map(|line| line.as_bytes())
            .collect();
        let rooms = [
            [lines[0][3], b'D', b'D', lines[1][3]],
            [lines[0][5], b'C', b'B', lines[1][5]],
            [lines[0][7], b'B', b'A', lines[1][7]],
            [lines[0][9], b'A', b'C', lines[1][9]],
        ];
        State { corridor, rooms }
    }
}

/// this represent the ids in the corridor directly above the rooms
const CORR_ROOMS_IDX: [usize; 4] = [2, 4, 6, 8];

impl<const N: usize> State<N> {
    fn print(&self) {
        println!(
            "{}",
            self.corridor.iter().map(|b| *b as char).collect::<String>()
        );

        for i in 0..self.rooms[0].len() {
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

    fn get_all_valid_moves(&self) -> Vec<(usize, State<N>)> {
        let mut moves = Vec::new();
        // === Moving parts from corridor to rooms ===
        for corr_idx in 0..self.corridor.len() {
            if self.corridor[corr_idx] == b'.' {
                continue;
            }

            let target_room_idx = (self.corridor[corr_idx] - b'A') as usize;
            // Check if the way in the corridor to the target room is free
            let room_corr_idx = CORR_ROOMS_IDX[target_room_idx];
            let (corr_start, corr_end) = if corr_idx > room_corr_idx {
                (room_corr_idx, corr_idx)
            } else {
                (corr_idx + 1, room_corr_idx + 1)
            };
            if (corr_start..corr_end).any(|i| self.corridor[i] != b'.') {
                continue;
            }

            // find the last free slot in the room
            let Some(free_room_idx) = (0..N)
                .take_while(|i| self.rooms[target_room_idx][*i] == b'.')
                .last()
            else {
                continue;
            };
            // Check is the room doesn't have any wrong items
            if (free_room_idx + 1..N)
                .any(|i| self.rooms[target_room_idx][i] != self.corridor[corr_idx])
            {
                continue;
            }

            moves.push(self.make_move(corr_idx, target_room_idx, free_room_idx));
        }

        // === Moving Parts out of the rooms ===
        for room_idx in 0..4 {
            let Some(non_free_idx) = (0..N).find(|i| self.rooms[room_idx][*i] != b'.') else {
                continue;
            };

            let room_corr_idx = CORR_ROOMS_IDX[room_idx];
            // Right possibilities
            let moves_right = (room_corr_idx..self.corridor.len())
                .take_while(|i| self.corridor[*i] == b'.')
                .filter(|i| !CORR_ROOMS_IDX.contains(i))
                .map(|i| self.make_move(i, room_idx, non_free_idx));
            moves.extend(moves_right);

            // Left possibilities
            let moves_left = (0..room_corr_idx)
                .rev()
                .take_while(|i| self.corridor[*i] == b'.')
                .filter(|i| !CORR_ROOMS_IDX.contains(i))
                .map(|i| self.make_move(i, room_idx, non_free_idx));
            moves.extend(moves_left);
        }

        moves
    }

    fn make_move(&self, idx: usize, room_idx: usize, room_depth: usize) -> (usize, State<N>) {
        let amph = if self.corridor[idx] == b'.' {
            self.rooms[room_idx][room_depth] as char
        } else {
            self.corridor[idx] as char
        };

        let room_corr_idx = CORR_ROOMS_IDX[room_idx];
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

fn calc_least_energy_folded(input: &str) -> usize {
    let state: State<2> = State::from(input);
    calc_least_energy(state)
}

fn calc_least_energy_unfolded(input: &str) -> usize {
    let state: State<4> = State::from(input);
    calc_least_energy(state)
}

fn calc_least_energy<const N: usize>(state: State<N>) -> usize {
    state.print();

    let mut energy_map: HashMap<State<N>, usize> = HashMap::new();
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
    let answer = calc_least_energy_folded(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "23");
    let answer = calc_least_energy_unfolded(input.as_str());

    println!("Part 1 answer is {answer}");
}

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
        assert_eq!(calc_least_energy_folded(INPUT), 12521);
        assert_eq!(calc_least_energy_unfolded(INPUT), 44169);
    }
}
