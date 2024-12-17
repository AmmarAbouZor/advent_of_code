use std::ops::BitXor;

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl From<&str> for Registers {
    fn from(value: &str) -> Self {
        let nums: Vec<_> = value
            .lines()
            .map(|line| {
                let num = line
                    .split_whitespace()
                    .skip(2)
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
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

    fn combo(self, reg: &Registers) -> i64 {
        match self.num {
            n @ 0..=3 => n as i64,
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
    fn apply(
        self,
        idx: usize,
        operand: Operand,
        reg: &mut Registers,
        out: &mut Vec<String>,
    ) -> usize {
        match self {
            Opcode::Adv => {
                let combo = operand.combo(reg);
                let denominator = 2_i64.pow(combo as u32);
                reg.a /= denominator;
            }
            Opcode::Bxl => {
                let literal = operand.literal();
                reg.b ^= literal as i64
            }
            Opcode::Bst => {
                let combo = operand.combo(&reg);
                reg.b = combo % 8;
            }
            Opcode::Jnz => {
                if reg.a != 0 {
                    return operand.literal() as usize;
                }
            }
            Opcode::Bxc => reg.b = reg.b.bitxor(reg.c),
            Opcode::Out => {
                let val = operand.combo(&reg) % 8;
                out.push(val.to_string());
            }
            Opcode::Bdv => {
                let combo = operand.combo(reg);
                let denominator = 2_i64.pow(combo as u32);
                reg.b = reg.a / denominator;
            }
            Opcode::Cdv => {
                let combo = operand.combo(reg);
                let denominator = 2_i64.pow(combo as u32);
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

fn run_insts(input: &str) -> String {
    let (mut regs, nums) = parse(input);
    let mut idx = 0;

    let mut out = Vec::new();
    while let Some(&opcode) = nums.get(idx) {
        let opcode = Opcode::from(opcode);
        let operand = Operand::from(nums[idx + 1]);
        idx = opcode.apply(idx, operand, &mut regs, &mut out);
    }

    out.join(",")
}

fn part_1(input: &'static str) {
    let out = run_insts(input);
    println!("Part 1 answer is '{out}'");
}

fn part_2(input: &'static str) {}

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

    #[test]
    fn test_solution() {
        let out = run_insts(INPUT);
        assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");

        assert_eq!(2 ^ 10, 8);
    }
}

