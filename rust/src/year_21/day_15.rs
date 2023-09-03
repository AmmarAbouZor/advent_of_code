use std::collections::BinaryHeap;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<i32>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        Grid { cells }
    }
}

impl Grid {
    fn calc_lowest_risk(&self) -> i32 {
        let width = self.cells[0].len();
        let height = self.cells.len();

        let mut dist = vec![vec![i32::MAX; width]; height];

        let mut stack = BinaryHeap::new();

        // Start point
        stack.push((0, 0, 0));

        while let Some((cost, row, col)) = stack.pop() {
            if (row, col) == (width - 1, height - 1) {
                return -cost;
            }

            if -cost > dist[row][col] {
                continue;
            }

            for (row_next, col_next) in [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col + 1),
                (row, col.wrapping_sub(1)),
            ] {
                let cost_next = match self.cells.get(row_next).and_then(|r| r.get(col_next)) {
                    Some(risk) => -cost + risk,
                    None => continue,
                };

                if cost_next < dist[row_next][col_next] {
                    stack.push((-cost_next, row_next, col_next));
                    dist[row_next][col_next] = cost_next;
                }
            }
        }

        unreachable!()
    }
}

fn part_1() {
    let input = read_text_from_file("21", "15");
    let grid = Grid::from(input.as_str());

    let answer = grid.calc_lowest_risk();

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

    const INPUT: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part_1() {
        let grid = Grid::from(INPUT);

        assert_eq!(grid.calc_lowest_risk(), 40);
    }
}
