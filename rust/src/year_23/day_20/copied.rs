// This is a copied soltuion with small changs so it fits my input.
// TODO: Rewrite this after understanding the math behind it.
// Important Links:
// https://www.reddit.com/r/adventofcode/comments/18mmfxb/comment/ke5v9bb/?utm_source=share&utm_medium=web2x&context=3
// https://www.electronics-tutorials.ws/counter/mod-counters.html
// https://github.com/happyhacks/aoc2023-rs/blob/master/day20b/src/main.rs

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

#[derive(PartialEq, Clone, Copy, Default, Debug, Hash, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Pulse {
    fn invert(&self) -> Pulse {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

impl std::ops::Not for Pulse {
    type Output = Pulse;
    fn not(self) -> Pulse {
        self.invert()
    }
}

#[derive(Default, Clone, Debug)]
struct FlipFlop {
    state: Pulse,
}

impl Module for FlipFlop {
    fn tick(&mut self, signal: Pulse, _: String) {
        if signal == Pulse::Low {
            self.state = !self.state;
        }
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
    fn cont(&self, signal: Pulse) -> bool {
        if signal == Pulse::High {
            return false;
        }
        true
    }
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    state: Pulse,
}

impl Module for Conjunction {
    fn tick(&mut self, signal: Pulse, name: String) {
        self.inputs.insert(name, signal);
        self.state = match self.inputs.iter().all(|(_, &p)| p == Pulse::High) {
            true => Pulse::Low,
            false => Pulse::High,
        };
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
    fn account(&mut self, name: String) {
        self.inputs.insert(name, Pulse::Low);
    }
}
#[derive(Default, Clone, Debug)]
struct Broadcaster {
    state: Pulse,
}

impl Module for Broadcaster {
    fn tick(&mut self, signal: Pulse, _: String) {
        self.state = signal;
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
}

trait Module: Debug {
    fn tick(&mut self, signal: Pulse, name: String);
    fn get_state(&self) -> Pulse;
    fn account(&mut self, _: String) {}
    fn cont(&self, _: Pulse) -> bool {
        true
    }
}

#[allow(clippy::all)]
pub fn solve_part_2(input: &'static str) -> usize {
    let mut g: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut e: HashMap<String, Vec<String>> = HashMap::new();
    input.trim_end().lines().for_each(|l| {
        let (src, dst) = l.split_once(" -> ").unwrap();
        let (comp, name): (Box<dyn Module>, &str) = match src.split_at(1) {
            ("%", src) => (Box::new(FlipFlop::default()), src),
            ("&", src) => (Box::new(Conjunction::default()), src),
            ("b", _) => (Box::new(Broadcaster::default()), "broadcaster"),
            _ => unreachable!(),
        };
        let neigh = dst.split(", ").map(|s| s.to_string()).collect();
        assert!(g.insert(name.trim().to_string(), comp).is_none());
        e.insert(name.trim().to_string(), neigh);
    });
    for (k, v) in e.iter() {
        for i in v {
            g.get_mut(i).map(|n| n.account(k.clone()));
        }
    }
    let mut cycle = Vec::new();
    for i in 0..4096 {
        let mut q = VecDeque::new();
        q.push_back(("broadcaster".to_string(), Pulse::Low));
        while let Some((name, signal)) = q.pop_front() {
            for n in e.get(&name).ok_or("Invalid module name").unwrap() {
                // dn is the last conjunction
                if n == "rx" || n == "output" || n == "dn" {
                    if signal == Pulse::High {
                        cycle.push(i + 1usize);
                    }
                    continue;
                }
                let neigh = g.get_mut(n).unwrap();
                if !neigh.cont(signal) {
                    continue;
                }
                neigh.tick(signal, name.clone());
                q.push_back((n.to_string(), neigh.get_state()));
            }
        }
    }

    lcm(&cycle)
}

/// calculates Lowest Common Multiple of a group on numbers
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
