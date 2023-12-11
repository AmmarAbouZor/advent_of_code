use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, Hash)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn calc_distance(&self, other: &Galaxy) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn get_expanded(input: &str, factor: usize) -> Vec<Galaxy> {
    let mut row_offset = 0;

    // Parse with row offset directly
    let mut galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let galaxies: Vec<_> = line
                .chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(|(col, _)| Galaxy::new(row + row_offset, col))
                .collect();
            if galaxies.is_empty() {
                row_offset += factor;
            }
            galaxies
        })
        .collect();

    // Add column Offset
    galaxies.sort_unstable_by_key(|gal| gal.col);

    let mut col_offset = 0;
    let mut last_col = 0_usize;

    let mut expanded_galaxies = Vec::with_capacity(galaxies.len());

    for mut galaxy in galaxies {
        let col_diff = galaxy.col.saturating_sub(last_col).saturating_sub(1);
        col_offset += col_diff * factor;
        last_col = galaxy.col;
        galaxy.col += col_offset;
        expanded_galaxies.push(galaxy);
    }

    expanded_galaxies
}

fn calc_paths_sum(input: &str, factor: usize) -> usize {
    let galaxies = get_expanded(input, factor);

    let mut sum = 0;

    for (idx, gal) in galaxies.iter().enumerate() {
        for other_gal in galaxies.iter().skip(idx + 1) {
            sum += gal.calc_distance(other_gal);
        }
    }

    sum
}

fn part_1(input: &str) {
    let answer = calc_paths_sum(input, 1);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_paths_sum(input, 999999);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "11");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_solution() {
        assert_eq!(calc_paths_sum(INPUT, 1), 374);
        assert_eq!(calc_paths_sum(INPUT, 9), 1030);
        assert_eq!(calc_paths_sum(INPUT, 99), 8410);
    }
}

