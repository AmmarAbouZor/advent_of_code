#![allow(warnings, unused)]

use std::collections::HashMap;

const START_VALUE: usize = 20151125;
const TARGET_ROW: usize = 3010;
const TARGET_COL: usize = 3019;

fn get_next_value(current: usize) -> usize {
    (current * 252533) % 33554393
}

fn find_value_by(row: usize, col: usize) -> usize {
    let mut last_value = START_VALUE;
    let mut row_index = 2;
    loop {
        for col_index in 1..=row_index {
            last_value = get_next_value(last_value);
            let current_row = row_index + 1 - col_index;
            if (col_index == col && current_row == row) {
                return last_value;
            }
        }

        row_index += 1;
    }
}

fn part_1() {
    let code = find_value_by(TARGET_ROW, TARGET_COL);

    println!("part_1: Code is {code}");
}

pub fn run() {
    part_1();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_next_value() {
        assert_eq!(get_next_value(20151125), 31916031);
        assert_eq!(get_next_value(31916031), 18749137);
    }

    #[test]
    fn test_find_value_by() {
        assert_eq!(find_value_by(2, 1), 31916031);
        assert_eq!(find_value_by(1, 2), 18749137);
        assert_eq!(find_value_by(3, 2), 8057251);
        assert_eq!(find_value_by(2, 3), 16929656);
    }
}
