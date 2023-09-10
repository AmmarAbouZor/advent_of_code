use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct SnailNum {
    num: u32,
    depth: u32,
}

impl SnailNum {
    fn new(num: u32, depth: u32) -> Self {
        Self { num, depth }
    }
}

fn line_to_nums(line: &str) -> Vec<SnailNum> {
    let mut depth = 0;
    let mut nums = Vec::new();
    for ch in line.chars() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            ch => {
                let num = ch.to_digit(10).unwrap();
                nums.push(SnailNum::new(num, depth));
            }
        }
    }

    nums
}

fn add_snails(mut s_1: Vec<SnailNum>, mut s_2: Vec<SnailNum>) -> Vec<SnailNum> {
    s_1.append(&mut s_2);

    s_1.iter_mut().for_each(|num| num.depth += 1);

    s_1
}

fn calc_magnitude(input: &str) -> usize {
    let line = input.lines().next().unwrap();
    let snails = line_to_nums(line);
    println!("{line}");
    dbg!(snails);

    todo!()
}

fn part_1() {
    let input = read_text_from_file("21", "18");
    let answer = calc_magnitude(input.as_str());

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_magnitude(INPUT), 4140);
    }
}

