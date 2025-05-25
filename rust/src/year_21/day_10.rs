use std::collections::BinaryHeap;

use crate::utls::read_text_from_file;

fn check_illegal(line: &str) -> Option<char> {
    let mut stack = Vec::with_capacity(line.len());

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            _ => match (ch, stack.pop()) {
                (')', Some('(')) => {}
                (']', Some('[')) => {}
                ('}', Some('{')) => {}
                ('>', Some('<')) => {}
                (ch, _) => return Some(ch),
            },
        }
    }

    None
}

fn get_char_points_error(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn calc_error_score(input: &str) -> usize {
    input
        .lines()
        .flat_map(check_illegal)
        .map(get_char_points_error)
        .sum()
}

fn get_complition(line: &str) -> Vec<char> {
    let mut stack = Vec::with_capacity(line.len());

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            _ => match (ch, stack.pop()) {
                (')', Some('(')) => {}
                (']', Some('[')) => {}
                ('}', Some('{')) => {}
                ('>', Some('<')) => {}
                _ => unreachable!(),
            },
        }
    }

    stack
        .into_iter()
        .rev()
        .map(|ch| match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        })
        .collect()
}

fn calc_complition_score(complition: Vec<char>) -> usize {
    let mut score = 0;

    for ch in complition {
        score *= 5;

        score += match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        }
    }

    score
}

fn calc_overall_comp_score(input: &str) -> usize {
    let mut heap: BinaryHeap<_> = input
        .lines()
        .filter(|line| check_illegal(line).is_none())
        .map(get_complition)
        .map(calc_complition_score)
        .collect();

    for _ in 0..(heap.len() / 2) {
        let _ = heap.pop().unwrap();
    }

    heap.pop().unwrap()
}

fn part_1() {
    let input = read_text_from_file("21", "10");
    let answer = calc_error_score(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "10");
    let answer = calc_overall_comp_score(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_syntax() {
        assert_eq!(calc_error_score(INPUT), 26397);
        assert_eq!(calc_overall_comp_score(INPUT), 288957);
    }
}
