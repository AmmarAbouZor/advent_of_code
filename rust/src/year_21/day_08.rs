use std::{collections::HashMap, usize};

use crate::utls::read_text_from_file;

fn calc_uniqe_digits(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .flat_map(|chunks| chunks.split_whitespace())
        .filter(|num| matches!(num.len(), 2 | 3 | 4 | 7))
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Segments(u8);

impl From<&str> for Segments {
    fn from(value: &str) -> Self {
        let mut segments = 0;
        for &b in value.as_bytes() {
            segments |= 0b1 << (b - b'a') as usize;
        }

        Self(segments)
    }
}

fn calc_line_output(line: &str) -> usize {
    let mut num_segment_map = HashMap::with_capacity(10);

    let (patterns, output) = line.split_once(" | ").unwrap();

    let patterns: Vec<_> = patterns.split_whitespace().collect();

    let mut zero_six_nine = Vec::with_capacity(3);
    let mut two_three_five = Vec::with_capacity(3);

    for pattern in patterns {
        match pattern.len() {
            2 => {
                num_segment_map.insert(1, Segments::from(pattern));
            }
            3 => {
                num_segment_map.insert(7, Segments::from(pattern));
            }
            4 => {
                num_segment_map.insert(4, Segments::from(pattern));
            }
            7 => {
                num_segment_map.insert(8, Segments::from(pattern));
            }
            6 => zero_six_nine.push(Segments::from(pattern)),
            5 => two_three_five.push(Segments::from(pattern)),
            _ => {}
        };
    }
    const EIGHT: u8 = 0b0111_1111;

    // three
    let three = if two_three_five[0].0 | two_three_five[1].0 == EIGHT {
        two_three_five.pop().unwrap()
    } else if two_three_five[0].0 | two_three_five[2].0 == EIGHT {
        two_three_five.remove(1)
    } else {
        two_three_five.remove(0)
    };

    let mut two_five = two_three_five;

    // nine
    let nine_idx = zero_six_nine
        .iter()
        .position(|segment| segment.0 | three.0 != EIGHT)
        .unwrap();
    let nine = zero_six_nine.remove(nine_idx);

    let mut zero_six = zero_six_nine;

    // two & five
    let two_or_five = two_five.pop().unwrap();
    if two_or_five.0 | nine.0 == EIGHT {
        num_segment_map.insert(2, two_or_five);
        num_segment_map.insert(5, two_five.pop().unwrap());
    } else {
        num_segment_map.insert(5, two_or_five);
        num_segment_map.insert(2, two_five.pop().unwrap());
    }

    // zero & six
    let zero_or_six = zero_six.pop().unwrap();
    if zero_or_six.0 | num_segment_map.get(&5).unwrap().0 == EIGHT {
        num_segment_map.insert(0, zero_or_six);
        num_segment_map.insert(6, zero_six.pop().unwrap());
    } else {
        num_segment_map.insert(6, zero_or_six);
        num_segment_map.insert(0, zero_six.pop().unwrap());
    }

    num_segment_map.insert(3, three);
    num_segment_map.insert(9, nine);

    let seg_num_map: HashMap<_, _> = num_segment_map
        .into_iter()
        .map(|(num, seg)| (seg, num))
        .collect();

    let mut pow = 1000;
    let mut sum = 0;
    for chunk in output.split_whitespace() {
        let segments = Segments::from(chunk);
        let &num = seg_num_map.get(&segments).unwrap();
        sum += num * pow;
        pow /= 10;
    }

    sum
}

fn calc_outputs(input: &str) -> usize {
    input.lines().map(calc_line_output).sum()
}

fn part_1() {
    let input = read_text_from_file("21", "08");
    let answer = calc_uniqe_digits(&input);

    println!("Part 1 answer is {answer}");
}
fn part_2() {
    let input = read_text_from_file("21", "08");
    let answer = calc_outputs(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_calc_nums() {
        assert_eq!(calc_uniqe_digits(INPUT), 26);
        assert_eq!(calc_outputs(INPUT), 61229);
    }
}

