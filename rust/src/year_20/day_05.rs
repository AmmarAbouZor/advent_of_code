use std::collections::BTreeSet;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Seat<'a> {
    rows: &'a [u8],
    cols: &'a [u8],
}

impl<'a> From<&'a str> for Seat<'a> {
    fn from(value: &'a str) -> Self {
        let bytes = value.as_bytes();

        let rows = &bytes[..7];
        let cols = &bytes[7..];

        Seat { rows, cols }
    }
}

impl Seat<'_> {
    pub fn seat_id(&self) -> usize {
        // rows
        let (mut min, mut max) = (0, 127);
        for byte in self.rows {
            match byte {
                b'F' => max = (min + max) / 2,
                b'B' => min = (min + max) / 2 + 1,
                invalid => unreachable!("Rows invalid: {invalid}"),
            }
        }
        let row = min;

        // horizontal
        let (mut min, mut max) = (0, 7);
        for byte in self.cols {
            match byte {
                b'L' => max = (min + max) / 2,
                b'R' => min = (min + max) / 2 + 1,
                invalid => unreachable!("Cols invalid: {invalid}"),
            }
        }
        let col = min;

        row * 8 + col
    }
}

fn calc_max_seat_id(input: &str) -> usize {
    input
        .lines()
        .map(Seat::from)
        .map(|seat| seat.seat_id())
        .max()
        .unwrap()
}

fn get_missing_seat_id(input: &str) -> usize {
    let seats: BTreeSet<usize> = input
        .lines()
        .map(Seat::from)
        .map(|seat| seat.seat_id())
        .collect();

    let mut seats_iter = seats.iter();

    let mut last_id = *seats_iter.next().unwrap();

    for &id in seats_iter {
        if id - last_id != 1 {
            return id - 1;
        }
        last_id = id;
    }

    unreachable!()
}

fn part_1() {
    let input = read_text_from_file("20", "05");
    let answer = calc_max_seat_id(&input);

    println!("Part 1 answer is {answer}")
}

fn part_2() {
    let input = read_text_from_file("20", "05");
    let answer = get_missing_seat_id(&input);

    println!("Part 2 answer is {answer}")
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seat_id() {
        let seats_ids = [
            ("FBFBBFFRLR", 357),
            ("BFFFBBFRRR", 567),
            ("FFFBBBFRRR", 119),
            ("BBFFBBFRLL", 820),
        ];

        for (code, id) in seats_ids {
            let seat = Seat::from(code);
            assert_eq!(seat.seat_id(), id);
        }
    }
}
