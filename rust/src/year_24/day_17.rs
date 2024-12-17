use std::ops::BitXor;

#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl From<&str> for Registers {
    fn from(value: &str) -> Self {
        let nums: Vec<_> = value
            .lines()
            .map(|line| {
                let num = line.split_whitespace().nth(2).unwrap().parse().unwrap();
                num
            })
            .collect();

        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        Self { a, b, c }
    }
}

#[derive(Debug, Clone, Copy)]
struct Operand {
    num: u8,
}

impl From<u8> for Operand {
    fn from(num: u8) -> Self {
        Self { num }
    }
}

impl Operand {
    #[inline]
    fn literal(self) -> u8 {
        self.num
    }

    fn combo(self, reg: &Registers) -> u64 {
        match self.num {
            n @ 0..=3 => n as u64,
            4 => reg.a,
            5 => reg.b,
            6 => reg.c,
            7 => panic!("What the hell is reserved suppose to do!"),
            invaild => panic!("Invalid Opernad: {invaild}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        use Opcode as O;
        match value {
            0 => O::Adv,
            1 => O::Bxl,
            2 => O::Bst,
            3 => O::Jnz,
            4 => O::Bxc,
            5 => O::Out,
            6 => O::Bdv,
            7 => O::Cdv,
            invalid => panic!("Invalid Opcode: {invalid}"),
        }
    }
}

impl Opcode {
    fn apply(self, idx: usize, operand: Operand, reg: &mut Registers, out: &mut Vec<u8>) -> usize {
        match self {
            Opcode::Adv => {
                let combo = operand.combo(reg);
                let denominator = 2_u64.pow(combo as u32);
                reg.a /= denominator;
            }
            Opcode::Bxl => {
                let literal = operand.literal();
                reg.b ^= literal as u64
            }
            Opcode::Bst => {
                let combo = operand.combo(reg);
                reg.b = combo % 8;
            }
            Opcode::Jnz => {
                if reg.a != 0 {
                    return operand.literal() as usize;
                }
            }
            Opcode::Bxc => reg.b = reg.b.bitxor(reg.c),
            Opcode::Out => {
                let val = operand.combo(reg) % 8;
                out.push(val as u8);
            }
            Opcode::Bdv => {
                let combo = operand.combo(reg);
                let denominator = 2_u64.pow(combo as u32);
                reg.b = reg.a / denominator;
            }
            Opcode::Cdv => {
                let combo = operand.combo(reg);
                let denominator = 2_u64.pow(combo as u32);
                reg.c = reg.a / denominator;
            }
        }

        idx + 2
    }
}

fn parse(input: &str) -> (Registers, Vec<u8>) {
    let (regs, nums) = input.split_once("\n\n").unwrap();
    let regs = regs.into();
    let (_, nums) = nums.split_once(' ').unwrap();
    let nums = nums.trim().split(',').map(|n| n.parse().unwrap()).collect();

    (regs, nums)
}

fn calc_output(input: &str) -> String {
    let (regs, nums) = parse(input);
    let out = run_insts(regs, &nums);

    let out: Vec<_> = out.iter().map(|n| n.to_string()).collect();

    out.join(",")
}

fn run_insts(mut regs: Registers, nums: &[u8]) -> Vec<u8> {
    let mut idx = 0;

    let mut out = Vec::new();
    while let Some(&opcode) = nums.get(idx) {
        let opcode = Opcode::from(opcode);
        let operand = Operand::from(nums[idx + 1]);
        idx = opcode.apply(idx, operand, &mut regs, &mut out);
    }

    out
}

fn part_1(input: &'static str) {
    let out = calc_output(input);
    println!("Part 1 answer is '{out}'");
}

#[allow(unused)]
fn print_pattern(input: &str, start: u64) {
    let (mut regs, nums) = parse(input);

    regs.a = start;
    let out = run_insts(regs, &nums);
    println!("x: {start}, out: {out:?}");
}

fn find_reg_a(input: &str) -> u64 {
    let (regs, nums) = parse(input);

    // Note: Idea is taken from other solutions online.
    // The digits in the output seem to change after a fixed period. Each
    // period is the previous one times 8: [1, 8, 64, 512, ...].
    //
    // output:
    //
    // x: 0, out: [0]
    // x: 7, out: [0]
    // x: 8, out: [1, 0]
    // x: 63, out: [7, 0]
    // x: 64, out: [0, 1, 0]
    // x: 511, out: [7, 7, 0]
    // x: 512, out: [0, 0, 1, 0]

    // Try all factors of each period and compare their output.

    let mut factors = vec![0; nums.len()];

    loop {
        let mut init_a = 0;
        for (idx, fact) in factors.iter().enumerate() {
            init_a += 8u64.pow(idx as u32) * fact
        }

        let mut regs_clone = regs.clone();
        regs_clone.a = init_a;
        let out = run_insts(regs_clone, &nums);
        if out == nums {
            return init_a;
        }

        for i in (0..nums.len()).rev() {
            if out.len() < i {
                factors[i] += 1;
                break;
            }

            if out[i] != nums[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

fn part_2(input: &'static str) {
    let ans = find_reg_a(input);
    println!("Part 2 answer is {ans}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "17").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_solution() {
        let out = calc_output(INPUT);
        assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");

        let reg = find_reg_a(INPUT_2);
        assert_eq!(reg, 117440);

        print_pattern(INPUT_2, 0);
        print_pattern(INPUT_2, 7);
        print_pattern(INPUT_2, 8);
        print_pattern(INPUT_2, 63);
        print_pattern(INPUT_2, 64);
        print_pattern(INPUT_2, 511);
        print_pattern(INPUT_2, 512);
    }
}
