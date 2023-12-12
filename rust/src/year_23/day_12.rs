use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::utls::read_text_from_file;

#[derive(Debug, Copy, Clone)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for State {
    fn from(ch: char) -> Self {
        match ch {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            invaild => unreachable!("Invalid input: '{invaild}'"),
        }
    }
}

#[derive(Debug)]
struct Record {
    states: Vec<State>,
    records: Vec<u8>,
}

impl From<&str> for Record {
    fn from(line: &str) -> Self {
        let (states, records) = line.split_once(' ').unwrap();
        let states = states.chars().map(State::from).collect();
        let records = records.split(',').map(|num| num.parse().unwrap()).collect();

        Record { states, records }
    }
}

impl Record {
    fn expand(&mut self) {
        self.states.push(State::Unknown);
        let mut expaned_state = Vec::with_capacity(self.states.len() * 5);
        let mut expand_records = Vec::with_capacity(self.records.len() * 5);
        for _ in 0..5 {
            expaned_state.extend(self.states.iter());
            expand_records.extend(self.records.iter());
        }
        self.states = expaned_state;
        self.records = expand_records;
    }

    fn get_arrangements(&self) -> usize {
        // I kept this for reference only as I went with the recursion solution with parallelism
        // Self::get_possibilites(&self.states)
        //     .iter()
        //     .map(|states| Self::get_groups(states))
        //     .filter(|arr| self.records == *arr)
        //     .count()

        let mut count = 0;
        let states = Vec::new();
        self.solve_rec(0, states, &mut count);

        count
    }

    fn solve_rec(&self, idx: usize, mut states: Vec<State>, count: &mut usize) {
        if idx == self.states.len() {
            if *Self::get_groups(&states) == self.records {
                *count += 1;
            }
            return;
        }

        match self.states[idx] {
            valid_state @ (State::Operational | State::Damaged) => {
                states.push(valid_state);
                self.solve_rec(idx + 1, states, count);
            }
            State::Unknown => {
                let mut clone = states.clone();
                clone.push(State::Operational);
                self.solve_rec(idx + 1, clone, count);
                states.push(State::Damaged);
                self.solve_rec(idx + 1, states, count);
            }
        }
    }

    // I kept this for reference only as I went with the recursion solution with parallelism
    #[allow(unused)]
    fn get_possibilites(unsolved_states: &[State]) -> Vec<Vec<State>> {
        let mut posses = vec![Vec::new()];

        for unsolv_state in unsolved_states {
            match unsolv_state {
                valid_state @ (State::Operational | State::Damaged) => {
                    for state in posses.iter_mut() {
                        state.push(*valid_state);
                    }
                }
                State::Unknown => {
                    let mut damaged_posses = posses.clone();
                    for state in posses.iter_mut() {
                        state.push(State::Operational);
                    }

                    for state in damaged_posses.iter_mut() {
                        state.push(State::Damaged);
                    }

                    posses.extend(damaged_posses);
                }
            }
        }

        posses
    }

    fn get_groups(states: &[State]) -> Vec<u8> {
        let mut groups = vec![0];

        for state in states {
            match state {
                State::Damaged => {
                    *groups.last_mut().unwrap() += 1;
                }
                State::Operational => {
                    if *groups.last().unwrap() != 0 {
                        groups.push(0);
                    }
                }
                State::Unknown => unreachable!("This function shouldn't deal with unknown cases"),
            }
        }

        groups.retain(|num| *num != 0);

        groups
    }
}

fn calc_arr_sum(input: &str) -> usize {
    let maps: Vec<_> = input.lines().map(Record::from).collect();

    maps.par_iter()
        .map(|record| record.get_arrangements())
        .sum()
}

fn calc_expand_sum(input: &str) -> usize {
    // This can't work it need a dynamic programming solution
    let maps: Vec<_> = input
        .lines()
        .map(Record::from)
        .map(|mut record| {
            record.expand();
            record
        })
        .collect();

    maps.par_iter()
        .map(|record| record.get_arrangements())
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_arr_sum(input);

    println!("Part 1 asnwer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_expand_sum(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "12");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_solution() {
        assert_eq!(calc_arr_sum(INPUT), 21);
        assert_eq!(calc_expand_sum(INPUT), 525152);
    }
}

