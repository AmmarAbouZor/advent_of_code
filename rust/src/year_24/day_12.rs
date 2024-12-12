use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PosIsize {
    row: isize,
    col: isize,
}

impl PosIsize {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> &'static [Self] {
        &[Self::Up, Self::Right, Self::Down, Self::Left]
    }
    #[inline]
    fn next(self, pos: &Pos) -> Pos {
        match self {
            Dir::Up => Pos::new(pos.row.wrapping_sub(1), pos.col),
            Dir::Right => Pos::new(pos.row, pos.col + 1),
            Dir::Down => Pos::new(pos.row + 1, pos.col),
            Dir::Left => Pos::new(pos.row, pos.col.wrapping_sub(1)),
        }
    }
    #[inline]
    fn next_isize(self, pos: &PosIsize) -> PosIsize {
        match self {
            Dir::Up => PosIsize::new(pos.row - 1, pos.col),
            Dir::Right => PosIsize::new(pos.row, pos.col + 1),
            Dir::Down => PosIsize::new(pos.row + 1, pos.col),
            Dir::Left => PosIsize::new(pos.row, pos.col - 1),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_groups(grid: &[Vec<char>]) -> HashMap<char, Vec<Vec<Pos>>> {
    let mut groups: HashMap<char, Vec<Vec<Pos>>> = HashMap::new();

    let rows_count = grid.len();
    let cols_count = grid[0].len();

    for r in 0..rows_count {
        for c in 0..cols_count {
            let cur_char = grid[r][c];
            let cur_pos = Pos::new(r, c);
            let ch_sets = groups.entry(cur_char).or_default();
            let mut found = false;
            for dir in [Dir::Left, Dir::Up] {
                let next_pos = dir.next(&cur_pos);
                let next_ch = match grid.get(next_pos.row).and_then(|row| row.get(next_pos.col)) {
                    Some(ch) => *ch,
                    None => continue,
                };

                if next_ch != cur_char {
                    continue;
                }

                let grp = ch_sets.iter_mut().find(|g| g.contains(&next_pos)).unwrap();
                found = true;
                assert!(!grp.contains(&cur_pos));
                grp.push(cur_pos);
                break;
            }

            if !found {
                ch_sets.push(vec![cur_pos]);
            }
        }
    }

    // merge groups
    for (_ch, sets) in groups.iter_mut() {
        if sets.len() == 1 {
            continue;
        }

        let mut merge = true;
        while merge {
            merge = false;
            let mut cur_pos = 0;
            while cur_pos < sets.len() - 1 {
                let current = &sets[cur_pos];
                if current.is_empty() {
                    cur_pos += 1;
                    continue;
                }
                for n in 0..sets.len() {
                    if n == cur_pos {
                        continue;
                    }
                    let next = &sets[n];

                    if can_be_merged(current, next) {
                        merge = true;
                        let mut new_curr = current.to_vec();
                        new_curr.extend_from_slice(next);
                        sets[cur_pos] = new_curr;
                        sets[n] = Vec::new();
                        break;
                    }
                }
                cur_pos += 1;
            }
        }

        sets.retain(|set| !set.is_empty());
    }

    groups
}

fn can_be_merged(v1: &[Pos], v2: &[Pos]) -> bool {
    v1.iter().any(|p1| {
        Dir::all()
            .iter()
            .map(|dir| dir.next(p1))
            .any(|neighbor| v2.contains(&neighbor))
    })
}

fn group_area_perimeter(grid: &[Vec<char>], ch: char, grp: &Vec<Pos>) -> (usize, usize) {
    let area = grp.len();
    let mut perimeter = 0;
    for cur_pos in grp {
        for dir in Dir::all() {
            let next_pos = dir.next(cur_pos);
            match grid.get(next_pos.row).and_then(|row| row.get(next_pos.col)) {
                Some(&next_ch) if next_ch != ch => perimeter += 1,
                None => perimeter += 1,
                _ => {}
            }
        }
    }

    (area, perimeter)
}

fn calc_price(input: &str) -> usize {
    let grid = parse(input);
    let groups = get_groups(&grid);
    groups
        .into_iter()
        .map(|(char, sets)| {
            sets.iter()
                .map(|set| group_area_perimeter(&grid, char, set))
                // .inspect(|(area, perimeter)| println!("{char}: {area} * {perimeter}"))
                .map(|(area, perimter)| area * perimter)
                .sum::<usize>()
        })
        .sum()
}

fn part_1(input: &'static str) {
    let price = calc_price(input);
    println!("Part 1 answer is {price}")
}

fn group_area_side(grid: &[Vec<char>], ch: char, grp: &Vec<Pos>) -> (usize, usize) {
    let area = grp.len();
    let mut fences = Vec::new();
    for cur_pos in grp {
        let cur_pos = PosIsize::new(cur_pos.row as isize, cur_pos.col as isize);
        for &dir in Dir::all() {
            let next_pos = dir.next_isize(&cur_pos);
            if next_pos.row.is_negative() || next_pos.col.is_negative() {
                fences.push((dir, next_pos));
                continue;
            }
            match grid
                .get(next_pos.row as usize)
                .and_then(|row| row.get(next_pos.col as usize))
            {
                Some(&next_ch) if next_ch != ch => fences.push((dir, next_pos)),
                None => fences.push((dir, next_pos)),
                _ => {}
            };
        }
    }

    let mut count = 0;
    for (dir, fence) in fences.iter() {
        let dir = *dir;
        let is_horizontal = matches!(dir, Dir::Up | Dir::Down);

        if is_horizontal {
            let left = Dir::Left.next_isize(fence);
            if !fences.contains(&(dir, left)) {
                count += 1;
            }
        } else {
            // vertical
            let up = Dir::Up.next_isize(fence);
            if !fences.contains(&(dir, up)) {
                count += 1;
            }
        }
    }

    (area, count)
}

fn calc_fences(input: &str) -> usize {
    let grid = parse(input);
    let groups = get_groups(&grid);
    groups
        .into_iter()
        .map(|(char, sets)| {
            sets.iter()
                .map(|set| group_area_side(&grid, char, set))
                .map(|(area, perimter)| area * perimter)
                .sum::<usize>()
        })
        .sum()
}

fn part_2(input: &'static str) {
    let ans = calc_fences(input);
    println!("Part 2 answer is {ans}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "12").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_solution() {
        let price = calc_price(INPUT);
        assert_eq!(price, 1930);

        let fences = calc_fences(INPUT);
        assert_eq!(fences, 1206);
    }
}
