use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const LEN: usize = 5;

#[derive(Debug, Clone)]
struct Lock([u8; 5]);
#[derive(Debug, Clone)]
struct Key([u8; 5]);

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let items = input.trim().split("\n\n");
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for item in items {
        let mut counts = [0; LEN];
        item.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(idx, ch)| {
                if ch == '#' {
                    counts[idx] += 1;
                }
            })
        });
        if item.starts_with('#') {
            locks.push(Lock(counts));
        } else {
            keys.push(Key(counts));
        }
    }

    (locks, keys)
}

fn is_match(lock: &Lock, key: &Key) -> bool {
    // Limit is 7 instead of 5 because we are counting the start and end lines on each item
    lock.0.iter().zip(key.0.iter()).all(|(&l, &k)| l + k <= 7)
}

fn calc_matches(input: &str) -> usize {
    let (locks, keys) = parse(input);
    locks
        .par_iter()
        .map(|lock| keys.iter().filter(|key| is_match(lock, key)).count())
        .sum()
}

fn part_1(input: &'static str) {
    let ans = calc_matches(input);
    println!("Last day answer is {ans}");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "25").leak();
    part_1(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_solution() {
        let ans = calc_matches(INPUT);
        assert_eq!(ans, 3)
    }
}
