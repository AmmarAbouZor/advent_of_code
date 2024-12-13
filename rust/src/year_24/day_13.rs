use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Button {
    dx: isize,
    dy: isize,
}

impl From<&str> for Button {
    fn from(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        let dx = parts[2]
            .trim_start_matches("X+")
            .trim_end_matches(',')
            .parse()
            .unwrap();

        let dy = parts[3].trim_start_matches("Y+").parse().unwrap();

        Self { dx, dy }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl From<&str> for Pos {
    fn from(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        let x = parts[1]
            .trim_start_matches("X=")
            .trim_end_matches(',')
            .parse()
            .unwrap();
        let y = parts[2].trim_start_matches("Y=").parse().unwrap();

        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    but_a: Button,
    but_b: Button,
    target: Pos,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let but_a = lines.next().unwrap().into();
        let but_b = lines.next().unwrap().into();
        let target = lines.next().unwrap().into();

        Self {
            but_a,
            but_b,
            target,
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.split("\n\n").map(|chunk| chunk.into()).collect()
}

fn machine_tokens(machine: &Machine, max: isize) -> Option<usize> {
    let mut tokens = isize::MAX;

    for a in 1..=max {
        for b in 1..=max {
            let current_tokens = a * 3 + b;
            if current_tokens > tokens {
                continue;
            }

            let pos_x = machine.but_a.dx * a + machine.but_b.dx * b;
            let pos_y = machine.but_a.dy * a + machine.but_b.dy * b;

            if pos_x == machine.target.x && pos_y == machine.target.y {
                tokens = tokens.min(current_tokens)
            }
        }
    }

    if tokens == isize::MAX {
        None
    } else {
        Some(tokens as usize)
    }
}

fn normal_calc_min_tokens(input: &str) -> usize {
    let machines = parse(input);
    machines
        .par_iter()
        .filter_map(|machine| machine_tokens(machine, 100))
        .sum()
}

fn part_1(input: &'static str) {
    let ans = normal_calc_min_tokens(input);
    println!("Part 1 answer is {ans}");
}

fn huge_calc_min_tokens(input: &str) -> usize {
    let mut machines = parse(input);
    machines.iter_mut().for_each(|m| {
        m.target.x += 10000000000000;
        m.target.y += 10000000000000;
    });
    machines.par_iter().filter_map(machine_tokens_denom).sum()
}

// Machine input forms an equation: `target = x*but_a + y*but_b`
// This is can be solved in the simple math way
fn machine_tokens_denom(machine: &Machine) -> Option<usize> {
    let denom = machine.but_a.dx * machine.but_b.dy - machine.but_a.dy * machine.but_b.dx;
    if denom == 0 {
        return None;
    }

    let num1 = machine.but_a.dx * machine.target.y - machine.but_a.dy * machine.target.x;
    let num2 = machine.but_b.dy * machine.target.x - machine.but_b.dx * machine.target.y;

    if num1 % denom != 0 || num2 % denom != 0 {
        return None;
    }

    let a_count = num1 / denom;
    let b_count = num2 / denom;

    if a_count < 0 || b_count < 0 {
        return None;
    }

    let res = a_count + 3 * b_count;

    Some(res as usize)
}

fn part_2(input: &'static str) {
    let ans = huge_calc_min_tokens(input);
    println!("Part 2 answer is {ans}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "13").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_solution() {
        let tokens = normal_calc_min_tokens(INPUT);
        assert_eq!(tokens, 480);

        let mut machines = parse(INPUT);
        machines.iter_mut().for_each(|m| {
            m.target.x += 10000000000000;
            m.target.y += 10000000000000;
        });

        assert!(machine_tokens_denom(&machines[0]).is_none());
        assert!(machine_tokens_denom(&machines[1]).is_some());
        assert!(machine_tokens_denom(&machines[2]).is_none());
        assert!(machine_tokens_denom(&machines[3]).is_some());
    }
}
