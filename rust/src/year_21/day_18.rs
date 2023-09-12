use std::fmt::Display;

use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    OpenBracket,
    CloseBracket,
    Comma,
    Num(u32),
}

impl From<char> for Symbol {
    fn from(char: char) -> Self {
        use Symbol as S;
        match char {
            '[' => S::OpenBracket,
            ']' => S::CloseBracket,
            ',' => S::Comma,
            num => S::Num(num.to_digit(10).unwrap()),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::OpenBracket => write!(f, "["),
            Symbol::CloseBracket => write!(f, "]"),
            Symbol::Comma => write!(f, ","),
            Symbol::Num(num) => write!(f, "{}", num),
        }
    }
}

fn symbols_to_string(nums: &[Symbol]) -> String {
    nums.iter().map(ToString::to_string).collect()
}

fn parse_line(line: &str) -> Vec<Symbol> {
    line.chars().map(Symbol::from).collect()
}

#[inline]
fn do_explode(nums: &mut Vec<Symbol>) -> bool {
    use Symbol as S;
    let mut depth = 0;
    let expl_start_idx = nums.iter().position(|num| {
        match num {
            S::OpenBracket => {
                depth += 1;
            }
            S::CloseBracket => {
                depth -= 1;
            }
            S::Comma => {}
            S::Num(_) => {}
        };
        depth == 5
    });

    let expl_start_idx = match expl_start_idx {
        Some(idx) => idx,
        None => return false,
    };

    // [ left , right ]
    let left_idx = expl_start_idx + 1;

    let right_idx = left_idx + 2;

    assert!(matches!(&nums[left_idx], S::Num(_)));
    assert!(matches!(&nums[right_idx], S::Num(_)));

    let S::Num(left_num) = nums[left_idx] else {
        unreachable!()
    };

    let S::Num(right_num) = nums[right_idx] else {
        unreachable!()
    };

    if let Some(S::Num(prev_left)) = nums[..expl_start_idx]
        .iter_mut()
        .rfind(|num| matches!(num, S::Num(_)))
    {
        *prev_left += left_num;
    }

    if let Some(S::Num(next_right)) = nums[right_idx + 1..]
        .iter_mut()
        .find(|num| matches!(num, S::Num(_)))
    {
        *next_right += right_num;
    }

    nums[expl_start_idx] = S::Num(0);

    // Remove the range
    nums.drain(expl_start_idx + 1..expl_start_idx + 5)
        .for_each(|_| ());

    true
}

#[inline]
fn do_splite(nums: &mut Vec<Symbol>) -> bool {
    use Symbol as S;
    let Some(split_idx) = nums.iter().position(|n| {
        if let S::Num(num) = n {
            *num >= 10
        } else {
            false
        }
    }) else {
        return false;
    };

    let S::Num(num) = nums[split_idx] else {
        unreachable!()
    };

    let left = num / 2;
    let right = num - left;

    nums.splice(
        split_idx..split_idx + 1,
        [
            S::OpenBracket,
            S::Num(left),
            S::Comma,
            S::Num(right),
            S::CloseBracket,
        ],
    );

    true
}

#[inline]
fn add_and_reduce(mut s_1: Vec<Symbol>, mut s_2: Vec<Symbol>) -> Vec<Symbol> {
    s_1.insert(0, Symbol::OpenBracket);
    s_1.push(Symbol::Comma);
    s_2.push(Symbol::CloseBracket);

    s_1.append(&mut s_2);

    let mut nums = s_1;

    let mut changed = true;
    while changed {
        // println!("{}", line_to_string(&nums));
        changed = false;
        if do_explode(&mut nums) {
            changed = true;
        } else if do_splite(&mut nums) {
            changed = true;
        }
    }

    nums
}

fn calc_magnitude(input: &str) -> usize {
    let mut calc_lines = input.lines().map(parse_line);
    let mut current_line = calc_lines.next().unwrap();
    for next_line in calc_lines {
        current_line = add_and_reduce(current_line, next_line);
        println!("{}", symbols_to_string(&current_line));
    }

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

    const BASIC_INPUT: &str = r"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";

    const INPUT_1: &str = r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    const INPUT_2: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_add_and_reduce() {
        let mut nums = BASIC_INPUT.lines().map(parse_line);
        let added_num = add_and_reduce(nums.next().unwrap(), nums.next().unwrap());

        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            symbols_to_string(&added_num)
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(calc_magnitude(INPUT_1), 129);
        // assert_eq!(calc_magnitude(INPUT_2), 4140);
    }
}
