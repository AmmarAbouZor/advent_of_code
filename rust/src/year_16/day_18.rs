use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Trap,
    Safe,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn from_prev(left: Tile, center: Tile, right: Tile) -> Self {
        match (left, center, right) {
            (Tile::Trap, Tile::Trap, Tile::Safe)
            | (Tile::Safe, Tile::Trap, Tile::Trap)
            | (Tile::Trap, Tile::Safe, Tile::Safe)
            | (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
            _ => Tile::Safe,
        }
    }
}

fn read_input() -> Vec<Tile> {
    read_text_from_file("16", "18")
        .chars()
        .map(|ch| ch.into())
        .collect()
}

fn calc_next_row(cur_row: &[Tile]) -> Vec<Tile> {
    let mut row = Vec::new();
    row.reserve(cur_row.len());

    row.push(Tile::from_prev(Tile::Safe, cur_row[0], cur_row[1]));
    for i in 1..cur_row.len() - 1 {
        row.push(Tile::from_prev(cur_row[i - 1], cur_row[i], cur_row[i + 1]));
    }

    row.push(Tile::from_prev(
        cur_row[cur_row.len() - 2],
        cur_row[cur_row.len() - 1],
        Tile::Safe,
    ));

    row
}

fn get_safe_tiles(rows_count: usize) -> usize {
    let mut rows = Vec::new();

    let first_row = read_input();
    rows.push(first_row);
    while rows.len() < rows_count {
        let last_row = rows.last().unwrap();
        let next_row = calc_next_row(last_row);
        rows.push(next_row);
    }

    rows.into_iter()
        .flatten()
        .filter(|t| *t == Tile::Safe)
        .count()
}

fn part_1() {
    let safe_count = get_safe_tiles(40);

    println!("part_1: num of safe tiles is {safe_count}");
}

fn part_2() {
    let safe_count = get_safe_tiles(400000);

    println!("part_2: num of safe tiles is {safe_count}");
}
pub fn run() {
    part_1();
    part_2();
}
