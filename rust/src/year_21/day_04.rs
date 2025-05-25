use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Board {
    cells: [[u32; 5]; 5],
}

impl From<&str> for Board {
    fn from(grid: &str) -> Self {
        let mut cells = [[0; 5]; 5];
        grid.lines().enumerate().for_each(|(row, line)| {
            line.split_whitespace()
                .enumerate()
                .for_each(|(col, num)| cells[row][col] = num.parse().unwrap());
        });

        Board { cells }
    }
}

impl Board {
    // check if the board has won and return the sum of the empty cells
    fn has_won(&self, nums: &[u32]) -> Option<u32> {
        let rows_check =
            || (0..5).any(|row| (0..5).all(|col| nums.contains(&self.cells[row][col])));

        let cols_check =
            || (0..5).any(|col| (0..5).all(|row| nums.contains(&self.cells[row][col])));

        if rows_check() || cols_check() {
            let sum = self
                .cells
                .iter()
                .flat_map(|row| row.iter())
                .filter(|num| !nums.contains(num))
                .sum();
            Some(sum)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let chunks: Vec<&str> = input.split_terminator("\n\n").collect();

    let nums = chunks[0]
        .split_terminator(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let boards = chunks.into_iter().skip(1).map(Board::from).collect();

    (nums, boards)
}

fn calc_score(input: &str) -> u32 {
    let (all_nums, boards) = parse_input(input);

    let mut nums = Vec::new();
    for num in all_nums {
        nums.push(num);
        if let Some(score) = boards.iter().flat_map(|board| board.has_won(&nums)).next() {
            return score * nums.last().unwrap();
        }
    }

    unreachable!()
}

fn calc_score_last(input: &str) -> u32 {
    let (all_nums, mut boards) = parse_input(input);

    let mut nums = Vec::new();
    for num in all_nums {
        nums.push(num);
        if boards.len() > 1 {
            boards.retain(|board| board.has_won(&nums).is_none())
        } else if let Some(score) = boards[0].has_won(&nums) {
            return score * nums.last().unwrap();
        }
    }

    unreachable!()
}

fn part_1() {
    let input = read_text_from_file("21", "04");
    let answer = calc_score(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "04");

    let answer = calc_score_last(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_score(INPUT), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_score_last(INPUT), 1924);
    }
}
