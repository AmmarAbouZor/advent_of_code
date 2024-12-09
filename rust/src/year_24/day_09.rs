fn parse(input: &str) -> Vec<u32> {
    input.chars().filter_map(|ch| ch.to_digit(10)).collect()
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    File,
    Empty,
}

fn expand(digits: &[u32]) -> Vec<Option<u32>> {
    let mut expanded = Vec::with_capacity(digits.len() * 10);
    let mut turn = Turn::File;
    let mut file_idx = 0;
    for dig in digits {
        let dig = (*dig) as usize;
        match turn {
            Turn::File => {
                expanded.extend(std::iter::repeat_n(Some(file_idx), dig));
                file_idx += 1;
            }
            Turn::Empty => {
                expanded.extend(std::iter::repeat_n(None, dig));
            }
        }

        turn = match turn {
            Turn::File => Turn::Empty,
            Turn::Empty => Turn::File,
        }
    }

    expanded
}

fn switch(layout: &mut [Option<u32>]) {
    let mut left = 0;
    for right in (0..layout.len()).rev() {
        if layout[right].is_none() {
            continue;
        }

        let empty_idx = layout.iter().skip(left).position(|e| e.is_none()).unwrap();
        left += empty_idx;
        if left > right {
            break;
        }
        layout.swap(left, right);
    }
}

fn checksum_from_switched(layout: &[Option<u32>]) -> u64 {
    layout
        .iter()
        .enumerate()
        .filter_map(|(idx, &n)| n.map(|n| (idx, n)))
        .map(|(idx, num)| idx as u64 * num as u64)
        .sum()
}

fn calc_checksum(input: &str) -> u64 {
    let digits = parse(input);
    let mut expanded = expand(&digits);
    switch(&mut expanded);
    checksum_from_switched(&expanded)
}

fn part_1(input: &'static str) {
    let ans = calc_checksum(input);
    println!("Part 1 answer is {ans}")
}

fn switch_whole(layout: &mut [Option<u32>]) {
    let mut right = layout.len() - 1;
    while right > 0 {
        if layout[right].is_none() {
            right -= 1;
            continue;
        }
        let mut start_right = right;
        while layout[right] == layout[start_right] && start_right > 0 {
            start_right -= 1;
        }

        let mut left = 0;
        loop {
            if left > start_right {
                break;
            }
            let Some(next_empty) = layout.iter().skip(left).position(|s| s.is_none()) else {
                break;
            };

            left += next_empty;
            if left > start_right {
                break;
            }
            let needed_empty_len = right - start_right;
            let end_left = left + needed_empty_len;
            if left > start_right {
                break;
            }
            if layout[left..end_left].iter().all(|s| s.is_none()) {
                let mut r = start_right;
                for l in left..end_left {
                    r += 1;
                    layout.swap(l, r);
                }
                assert!(r == right);
                break;
            }
            left += 1;
        }

        right = start_right;
    }
}

#[allow(unused)]
fn print_debug(items: &[Option<u32>]) {
    for item in items {
        match item {
            Some(d) => eprint!("({d})"),
            None => eprint!("."),
        }
    }
    eprintln!()
}

fn calc_checksum_whole(input: &str) -> u64 {
    let digits = parse(input);
    let mut expanded = expand(&digits);
    // print_debug(&expanded);

    switch_whole(&mut expanded);

    // print_debug(&expanded);
    checksum_from_switched(&expanded)
}

fn part_2(input: &'static str) {
    let ans = calc_checksum_whole(input);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "09").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_solution() {
        let chksm = calc_checksum(INPUT);
        assert_eq!(chksm, 1928);

        let chksm_whole = calc_checksum_whole(INPUT);
        assert_eq!(chksm_whole, 2858)
    }
}
