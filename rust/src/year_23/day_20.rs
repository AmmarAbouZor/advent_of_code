use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        input_map: Vec<(&'static str, Pulse)>,
    },
    BroadCast,
}

impl ModuleType {
    fn create_flipflop() -> ModuleType {
        ModuleType::FlipFlop { state: false }
    }

    fn create_conjunction() -> ModuleType {
        ModuleType::Conjunction {
            input_map: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    module_type: ModuleType,
    name: &'static str,
    targets: Vec<&'static str>,
}

impl From<&'static str> for Module {
    fn from(line: &'static str) -> Self {
        let (type_name, targets) = line.split_once(" -> ").unwrap();
        let (name, module_type) = match type_name {
            "broadcaster" => ("broadcaster", ModuleType::BroadCast),
            tname if tname.starts_with('%') => (&tname[1..], ModuleType::create_flipflop()),
            tname if tname.starts_with('&') => (&tname[1..], ModuleType::create_conjunction()),
            invalid => unreachable!("invalid input '{invalid}'"),
        };

        let targets = targets.split(", ").collect();

        Self {
            module_type,
            name,
            targets,
        }
    }
}

#[derive(Debug, Clone)]
struct PulseState {
    sender: &'static str,
    target: &'static str,
    pulse: Pulse,
}

impl PulseState {
    fn new(sender: &'static str, target: &'static str, pulse: Pulse) -> Self {
        Self {
            sender,
            target,
            pulse,
        }
    }
}

impl Module {
    fn apply(&mut self, sender: &str, pulse: Pulse) -> Vec<PulseState> {
        let pulse = match &mut self.module_type {
            ModuleType::FlipFlop { state } => match pulse {
                Pulse::Low => {
                    *state = !*state;
                    if *state {
                        Pulse::High
                    } else {
                        Pulse::Low
                    }
                }
                Pulse::High => return Vec::new(),
            },
            ModuleType::Conjunction { input_map } => {
                let conj = input_map.iter_mut().find(|m| m.0 == sender).unwrap();
                conj.1 = pulse;

                if input_map.iter().all(|(_, pu)| matches!(*pu, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            ModuleType::BroadCast => Pulse::Low,
        };

        self.targets
            .iter()
            .map(|t| PulseState::new(self.name, t, pulse))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Machine {
    modules: Vec<Module>,
}

impl Machine {
    fn fill_conj_initial(&mut self) {
        let conjs: Vec<&str> = self
            .modules
            .iter()
            .filter(|m| matches!(&m.module_type, ModuleType::Conjunction { input_map: _ }))
            .map(|m| m.name)
            .collect();

        let to_add: Vec<_> = self
            .modules
            .iter()
            .map(|m| {
                (
                    m.name,
                    m.targets
                        .iter()
                        .filter(|t| conjs.contains(t))
                        .cloned()
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, targets)| !targets.is_empty())
            .collect();

        for (sender, targets) in to_add {
            for target in targets {
                let modul = self.modules.iter_mut().find(|m| m.name == target).unwrap();
                let ModuleType::Conjunction { input_map } = &mut modul.module_type else {
                    panic!("Must be conjunction")
                };
                if input_map.iter().all(|inmap| inmap.0 != target) {
                    input_map.push((sender, Pulse::Low));
                }
            }
        }
    }

    fn apply_round(&mut self) -> (usize, usize) {
        let mut broadcaster = self
            .modules
            .iter()
            .find(|m| matches!(m.module_type, ModuleType::BroadCast))
            .unwrap()
            .clone();

        // input doesn't matter for broadcaster
        let mut pulses = broadcaster.apply("", Pulse::Low).clone();
        let mut count_low = 1;
        let mut count_high = 0;

        while !pulses.is_empty() {
            pulses.iter().for_each(|p| match p.pulse {
                Pulse::Low => count_low += 1,
                Pulse::High => count_high += 1,
            });
            let new_pulses = pulses
                .iter()
                .flat_map(|state| {
                    self.modules
                        .iter_mut()
                        .find(|m| m.name == state.target)
                        .map_or_else(Vec::new, |module| module.apply(state.sender, state.pulse))
                })
                .collect();

            pulses = new_pulses;
        }

        (count_low, count_high)
    }

    fn apply(&mut self) -> usize {
        self.fill_conj_initial();

        let initial_state = self.modules.clone();

        let mut counts = Vec::new();

        let target = 1000;

        while counts.len() < target {
            let score = self.apply_round();
            counts.push(score);
            if self.modules == initial_state {
                break;
            }
        }

        let summed =
            counts
                .iter()
                .cloned()
                .cycle()
                .take(target)
                .fold((0, 0), |(mut l, mut h), co| {
                    l += co.0;
                    h += co.1;
                    (l, h)
                });
        summed.0 * summed.1
    }

    fn apply_round_min(&mut self) -> bool {
        let mut broadcaster = self
            .modules
            .iter()
            .find(|m| matches!(m.module_type, ModuleType::BroadCast))
            .unwrap()
            .clone();

        // input doesn't matter for broadcaster
        let mut pulses = broadcaster.apply("", Pulse::Low).clone();

        let mut found = false;
        while !pulses.is_empty() {
            // dn is the last conjunction before rx
            if pulses
                .iter()
                .any(|p| matches!(p.pulse, Pulse::High) && matches!(p.target, "dn"))
            {
                found = true;
            }
            let new_pulses = pulses
                .iter()
                .flat_map(|state| {
                    self.modules
                        .iter_mut()
                        .find(|m| m.name == state.target)
                        .map_or_else(Vec::new, |module| module.apply(state.sender, state.pulse))
                })
                .collect();

            pulses = new_pulses;
        }

        found
    }

    fn apply_min(&mut self) -> usize {
        // TODO: Read the following article
        // https://www.electronics-tutorials.ws/counter/mod-counters.html

        self.fill_conj_initial();

        let mut counts = Vec::new();

        for i in 0..4096 {
            if self.apply_round_min() {
                counts.push(i + 1);
            }
        }

        lcm(&counts)
    }
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

impl From<&'static str> for Machine {
    fn from(input: &'static str) -> Self {
        let modules = input.lines().map(Module::from).collect();

        Self { modules }
    }
}

fn get_pulses_prod(input: &'static str) -> usize {
    let mut machine = Machine::from(input);
    machine.apply()
}

fn get_pulses_min(input: &'static str) -> usize {
    let mut machine = Machine::from(input);
    machine.apply_min()
}

fn part_1(input: &'static str) {
    let answer = get_pulses_prod(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &'static str) {
    let answer = get_pulses_min(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "20").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_solution() {
        assert_eq!(get_pulses_prod(INPUT_1), 32000000);
        assert_eq!(get_pulses_prod(INPUT_2), 11687500);
    }
}
