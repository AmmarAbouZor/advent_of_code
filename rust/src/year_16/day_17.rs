#[derive(Debug, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct State {
    pos: Pos,
    path: String,
}

fn get_solutions(input: &str) -> Vec<String> {
    let mut solutions = Vec::new();
    let mut states = vec![State::new(Pos::new(0, 0), String::new())];

    let is_char_valid = |ch: char| ch > 'a';

    while let Some(state) = states.pop() {
        if state.reached_target() {
            solutions.push(state.path);
            continue;
        }

        let hash: Vec<char> = format!(
            "{:x}",
            md5::compute(format!("{}{}", input, state.path).as_bytes())
        )
        .chars()
        .take(4)
        .collect();

        if is_char_valid(hash[0]) && state.pos.row > 0 {
            let mut new_state = state.clone();
            new_state.pos.row -= 1;
            new_state.path.push('U');
            states.push(new_state);
        }
        if is_char_valid(hash[1]) && state.pos.row < 3 {
            let mut new_state = state.clone();
            new_state.pos.row += 1;
            new_state.path.push('D');
            states.push(new_state);
        }
        if is_char_valid(hash[2]) && state.pos.col > 0 {
            let mut new_state = state.clone();
            new_state.pos.col -= 1;
            new_state.path.push('L');
            states.push(new_state);
        }
        if is_char_valid(hash[3]) && state.pos.col < 3 {
            let mut new_state = state.clone();
            new_state.pos.col += 1;
            new_state.path.push('R');
            states.push(new_state);
        }
    }

    solutions
}
impl State {
    fn new(pos: Pos, path: String) -> Self {
        Self { pos, path }
    }

    fn reached_target(&self) -> bool {
        self.pos.row == 3 && self.pos.col == 3
    }
}

pub fn run() {
    let mut solutions = get_solutions("bwnlcvfs");

    solutions.sort_by_key(|s| s.len());
    println!("shortest root is {}", solutions.first().unwrap());

    println!("longest root length is {}", solutions.last().unwrap().len());
}
