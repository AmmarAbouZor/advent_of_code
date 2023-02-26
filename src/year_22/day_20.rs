use crate::utls::read_text_from_file;

fn calc_sum(input: &str) -> isize {
    let coords: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut indexes: Vec<usize> = (0..coords.len()).collect();

    for (index, val) in coords.iter().enumerate() {
        let current_index = indexes.iter().position(|i| *i == index).unwrap();
        indexes.remove(current_index);

        let new_index = current_index as isize + val;
        let new_index = new_index.rem_euclid(indexes.len() as isize) as usize;

        indexes.insert(new_index, index);
    }

    // calc the difference
    // coord with value zero is our reference because it didn't move
    let origin = coords.iter().position(|val| *val == 0).unwrap();
    let offset = indexes.iter().position(|i| *i == origin).unwrap();

    let pos_1000 = indexes[(offset + 1000).rem_euclid(indexes.len())] % indexes.len();
    let pos_2000 = indexes[(offset + 2000).rem_euclid(indexes.len())] % indexes.len();
    let pos_3000 = indexes[(offset + 3000).rem_euclid(indexes.len())] % indexes.len();

    coords[pos_1000] + coords[pos_2000] + coords[pos_3000]
}

fn calc_sum_2(input: &str) -> isize {
    let coords: Vec<isize> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(|val| val * 811589153)
        .collect();

    let mut indexes: Vec<usize> = (0..coords.len()).collect();

    for _ in 0..10 {
        for (index, val) in coords.iter().enumerate() {
            let current_index = indexes.iter().position(|i| *i == index).unwrap();
            indexes.remove(current_index);

            let new_index = current_index as isize + val;
            let new_index = new_index.rem_euclid(indexes.len() as isize) as usize;

            indexes.insert(new_index, index);
        }
    }

    // calc the difference
    // coord with value zero is our reference because it didn't move
    let origin = coords.iter().position(|val| *val == 0).unwrap();
    let offset = indexes.iter().position(|i| *i == origin).unwrap();

    let pos_1000 = indexes[(offset + 1000).rem_euclid(indexes.len())] % indexes.len();
    let pos_2000 = indexes[(offset + 2000).rem_euclid(indexes.len())] % indexes.len();
    let pos_3000 = indexes[(offset + 3000).rem_euclid(indexes.len())] % indexes.len();

    coords[pos_1000] + coords[pos_2000] + coords[pos_3000]
}

fn part_1() {
    let input = read_text_from_file("22", "20");

    let answer = calc_sum(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("22", "20");

    let answer = calc_sum_2(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_sum(INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_sum_2(INPUT), 1623178306);
    }
}
