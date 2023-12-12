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
    fn get_arrangements(&self) -> usize {
        Self::get_possibilites(&self.states)
            .iter()
            .map(|states| Self::get_groups(states))
            .filter(|arr| self.records == *arr)
            .count()
    }

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
    input
        .lines()
        .map(Record::from)
        .map(|record| record.get_arrangements())
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_arr_sum(input);

    println!("Part 1 asnwer is {answer}");
}

fn part_2(input: &str) {}

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
    }
}

