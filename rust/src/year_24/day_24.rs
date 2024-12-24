use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    And,
    Or,
    Xor,
}

impl OpType {
    fn apply(self, n1: u8, n2: u8) -> u8 {
        match self {
            OpType::And => n1 & n2,
            OpType::Or => n1 | n2,
            OpType::Xor => n1 ^ n2,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    input1: String,
    input2: String,
    otype: OpType,
    output: String,
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let input1 = parts.next().unwrap().into();
        let otype = match parts.next().unwrap() {
            "AND" => OpType::And,
            "OR" => OpType::Or,
            "XOR" => OpType::Xor,
            invalid => panic!("Invalid OpType: {invalid}"),
        };

        let input2 = parts.next().unwrap().into();
        let output = parts.nth(1).unwrap().into();

        Self {
            input1,
            input2,
            otype,
            output,
        }
    }
}

fn parse(input: &str) -> (BTreeMap<String, u8>, Vec<Operation>) {
    let (known, operations) = input.split_once("\n\n").unwrap();
    let known: BTreeMap<String, u8> = known
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name.to_string(), val.parse().unwrap())
        })
        .collect();
    let operations = operations.lines().map(|line| line.into()).collect();

    (known, operations)
}

fn calc_outputz_sum(input: &str) -> usize {
    let (mut known, mut ops) = parse(input);

    while !ops.is_empty() {
        for i in 0..ops.len() {
            let op = &ops[i];
            let (Some(n1), Some(n2)) = (known.get(&op.input1), known.get(&op.input2)) else {
                continue;
            };
            let res = op.otype.apply(*n1, *n2);
            known.insert(op.output.clone(), res);
            ops.swap_remove(i);
            break;
        }
    }

    // Combine the results together.
    known
        .iter()
        .rev()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(_, val)| val)
        .fold(0_usize, |acc, &val| (acc << 1) | val as usize)
}

fn part_1(input: &'static str) {
    let ans = calc_outputz_sum(input);
    println!("Part 1 answer is {ans}");
}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "24").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_solution() {
        let ans = calc_outputz_sum(INPUT);
        assert_eq!(ans, 2024);
    }
}

