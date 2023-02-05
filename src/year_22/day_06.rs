use std::collections::HashSet;

use crate::utls::read_text_from_file;

fn find_marker_pos(input: &str, count: usize) -> usize {
    let input: Vec<char> = input.chars().collect();

    for i in count - 1..input.len() {
        let mut hash = HashSet::new();

        let mut distinct = true;
        for j in 0..count {
            if !hash.insert(input[i - j]) {
                distinct = false;
                break;
            }
        }

        if distinct {
            return i + 1;
        }
    }

    unreachable!("Pattern doesn't exist");
}

fn part_1() {
    let input = read_text_from_file("22", "06");
    let answer = find_marker_pos(&input, 4);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "06");
    let answer = find_marker_pos(&input, 14);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_marker_pos_4() {
        assert_eq!(find_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_find_marker_pos_14() {
        assert_eq!(find_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
