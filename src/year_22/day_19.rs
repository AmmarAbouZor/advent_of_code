use std::{collections::VecDeque, num::ParseIntError, str::FromStr, sync::mpsc, thread};

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct RobotCost {
    robot: Robot,
    cost_ore: usize,
    cost_clay: usize,
    cost_obsidian: usize,
}

impl FromStr for RobotCost {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let robot = parts.get(1).unwrap();
        let cost_1 = parts.get(4).unwrap().parse()?;
        let cost_2 = parts.get(7).and_then(|n| n.parse().ok());

        match *robot {
            "ore" => Ok(RobotCost::new(Robot::Ore, cost_1, 0, 0)),
            "clay" => Ok(RobotCost::new(Robot::Clay, cost_1, 0, 0)),
            "obsidian" => Ok(RobotCost::new(Robot::Obsidian, cost_1, cost_2.unwrap(), 0)),
            "geode" => Ok(RobotCost::new(Robot::Geode, cost_1, 0, cost_2.unwrap())),
            _ => unreachable!("invalid input"),
        }
    }
}

impl RobotCost {
    fn new(kind: Robot, cost_ore: usize, cost_clay: usize, cost_obsidian: usize) -> Self {
        Self {
            robot: kind,
            cost_ore,
            cost_clay,
            cost_obsidian,
        }
    }

    fn can_build(&self, state: &State, max_val: &RobotCost) -> bool {
        let built_robots = state.robots.iter().filter(|&&r| r == self.robot).count();
        let maxed_out = match self.robot {
            Robot::Ore => built_robots >= max_val.cost_ore,
            Robot::Clay => built_robots >= max_val.cost_clay,
            Robot::Obsidian => built_robots >= max_val.cost_obsidian,
            Robot::Geode => false,
        };

        !maxed_out
            && state.ore_count >= self.cost_ore
            && state.clay_count >= self.cost_clay
            && state.obsidian_count >= self.cost_obsidian
    }

    fn build(&self, state: &mut State) {
        // make sure program will crash if overflow happened
        state.ore_count = state.ore_count.checked_sub(self.cost_ore).unwrap();
        state.clay_count = state.clay_count.checked_sub(self.cost_clay).unwrap();
        state.obsidian_count = state
            .obsidian_count
            .checked_sub(self.cost_obsidian)
            .unwrap();
        state.robots.push(self.robot);
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    robot_costs: [RobotCost; 4],
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_sec, robots_sec) = s.split_once(':').unwrap();
        let id = id_sec.split_whitespace().nth(1).unwrap().parse()?;
        let robots: [RobotCost; 4] = robots_sec
            .split(". ")
            .flat_map(RobotCost::from_str)
            .collect::<Vec<RobotCost>>()
            .try_into()
            .unwrap();

        Ok(Blueprint {
            id,
            robot_costs: robots,
        })
    }
}

#[derive(Default, Debug, Clone)]
struct State {
    minutes: usize,
    ore_count: usize,
    clay_count: usize,
    obsidian_count: usize,
    geode_count: usize,
    robots: Vec<Robot>,
}

impl State {
    fn collect(&mut self) {
        self.robots.iter().for_each(|r| match r {
            Robot::Ore => self.ore_count += 1,
            Robot::Clay => self.clay_count += 1,
            Robot::Obsidian => self.obsidian_count += 1,
            Robot::Geode => self.geode_count += 1,
        });
    }

    fn estimate_best_result(&self, robot: Robot) -> usize {
        let element_count = match robot {
            Robot::Ore => self.ore_count,
            Robot::Clay => self.clay_count,
            Robot::Obsidian => self.obsidian_count,
            Robot::Geode => self.geode_count,
        };

        element_count
            + self.robots.iter().filter(|&&r| r == robot).count() * self.minutes
            + self.minutes * (self.minutes.checked_sub(1).unwrap()) / 2
    }
}

impl Blueprint {
    fn get_max_costs(&self) -> RobotCost {
        let mut max_ore = 0;
        let mut max_clay = 0;
        let mut max_obsidian = 0;

        self.robot_costs.iter().for_each(|c| {
            max_ore = max_ore.max(c.cost_ore);
            max_clay = max_clay.max(c.cost_clay);
            max_obsidian = max_obsidian.max(c.cost_obsidian);
        });

        RobotCost::new(Robot::Geode, max_ore, max_clay, max_obsidian)
    }

    fn get_max_geodes(&self, target: usize) -> usize {
        let state = State {
            robots: vec![Robot::Ore],
            minutes: target,
            ..Default::default()
        };

        let max_val = self.get_max_costs();

        let mut queue = VecDeque::from([state]);

        let mut max_geode = 0;

        while let Some(mut state) = queue.pop_back() {
            if state.minutes == 1 {
                state.collect();
                max_geode = max_geode.max(state.geode_count);
                continue;
            }

            if state.estimate_best_result(Robot::Geode) < max_geode {
                continue;
            }

            state.minutes -= 1;

            if self.robot_costs.last().unwrap().can_build(&state, &max_val) {
                state.collect();

                self.robot_costs.last().unwrap().build(&mut state);

                queue.push_back(state);
                continue;
            }

            // The don't build option
            let mut clone = state.clone();
            clone.collect();
            queue.push_back(clone);

            // Building robots possibilities
            self.robot_costs
                .iter()
                .filter(|r| r.can_build(&state, &max_val))
                .for_each(|r| {
                    let mut clone = state.clone();
                    clone.collect();
                    r.build(&mut clone);

                    queue.push_back(clone);
                });
        }

        max_geode
    }

    fn get_score(&self, target: usize) -> usize {
        self.id as usize * self.get_max_geodes(target)
    }
}

fn calc_part_1(input: &str) -> usize {
    let (tx, rx) = mpsc::channel();
    for blue_print in input.lines().flat_map(Blueprint::from_str) {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let score = blue_print.get_score(24);
            tx_clone.send(score).unwrap();
        });
    }

    drop(tx);

    rx.into_iter().sum()
}

fn calc_part_2(input: &str) -> usize {
    let (tx, rx) = mpsc::channel();

    for blue_print in input.lines().take(3).flat_map(Blueprint::from_str) {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let score = blue_print.get_max_geodes(32);
            tx_clone.send(score).unwrap();
        });
    }
    drop(tx);

    rx.into_iter().product()
}

fn part_1() {
    let input = read_text_from_file("22", "19");

    let answer = calc_part_1(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "19");

    let answer = calc_part_2(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_part_1(INPUT), 33);
    }
}
