use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    curr: Point,
    prev: Point,
    visited_slopes: Vec<Point>,
}

impl State {
    fn new(curr: Point, prev: Point, visited_slopes: Vec<Point>) -> Self {
        Self {
            curr,
            prev,
            visited_slopes,
        }
    }
}

#[derive(Debug)]
struct Maze {
    cells: Vec<&'static [u8]>,
}

impl From<&'static str> for Maze {
    fn from(value: &'static str) -> Self {
        let cells = value.lines().map(|line| line.as_bytes()).collect();

        Self { cells }
    }
}

impl Maze {
    fn reached_target(&self, p: &Point) -> bool {
        p.row == self.cells.len() - 1 && p.col == self.cells[0].len() - 2
    }

    fn get_content(&self, p: &Point) -> u8 {
        self.cells[p.row][p.col]
    }

    fn get_next_moves(&self, state: &State) -> Vec<Point> {
        let mut moves = Vec::new();

        let up = Point::new(state.curr.row - 1, state.curr.col);
        if up != state.prev && matches!(self.get_content(&up), b'.' | b'^') {
            moves.push(up);
        }

        let down = Point::new(state.curr.row + 1, state.curr.col);
        if down != state.prev && matches!(self.get_content(&down), b'.' | b'v') {
            moves.push(down);
        }

        let left = Point::new(state.curr.row, state.curr.col - 1);
        if left != state.prev && matches!(self.get_content(&left), b'.' | b'<') {
            moves.push(left);
        }

        let right = Point::new(state.curr.row, state.curr.col + 1);
        if right != state.prev && matches!(self.get_content(&right), b'.' | b'>') {
            moves.push(right);
        }

        moves
    }

    fn find_max_steps(&self) -> usize {
        // real input and the test has the first entrance
        let initial_state = State::new(Point::new(1, 1), Point::new(0, 1), Vec::new());
        let mut queue = VecDeque::new();
        // We've already made the first step
        queue.push_back((1, initial_state.clone()));

        let mut states_map: HashMap<State, usize> = HashMap::new();

        let mut max_steps = 0;

        while let Some((steps, state)) = queue.pop_front() {
            if self.reached_target(&state.curr) {
                max_steps = max_steps.max(steps);
                continue;
            }

            if let Some(saved_steps) = states_map.get_mut(&state) {
                if steps > *saved_steps {
                    *saved_steps = steps;
                } else {
                    continue;
                }
            }

            for p in self.get_next_moves(&state) {
                let mut clone = state.clone();
                let mut next_steps = steps + 2;
                let next_point = match self.get_content(&p) {
                    b'^' => {
                        if clone.visited_slopes.contains(&p) {
                            continue;
                        }
                        clone.visited_slopes.push(p);

                        Point::new(p.row - 1, p.col)
                    }
                    b'v' => {
                        if clone.visited_slopes.contains(&p) {
                            continue;
                        }
                        clone.visited_slopes.push(p);

                        Point::new(p.row + 1, p.col)
                    }
                    b'>' => {
                        if clone.visited_slopes.contains(&p) {
                            continue;
                        }
                        clone.visited_slopes.push(p);

                        Point::new(p.row, p.col + 1)
                    }
                    b'<' => {
                        if clone.visited_slopes.contains(&p) {
                            continue;
                        }
                        clone.visited_slopes.push(p);

                        Point::new(p.row, p.col - 1)
                    }
                    b'.' => {
                        next_steps -= 1;
                        p
                    }
                    invalid => unreachable!("Invalid byte: '{invalid}'"),
                };

                clone.prev = clone.curr;
                clone.curr = next_point;
                queue.push_back((next_steps, clone));
            }
        }

        max_steps
    }

    fn get_next_moves_no_slope(&self, state: &State) -> Vec<Point> {
        [
            Point::new(state.curr.row - 1, state.curr.col),
            Point::new(state.curr.row + 1, state.curr.col),
            Point::new(state.curr.row, state.curr.col - 1),
            Point::new(state.curr.row, state.curr.col + 1),
        ]
        .into_iter()
        .filter(|p| {
            *p != state.prev && matches!(self.get_content(p), b'.' | b'v' | b'^' | b'<' | b'>')
        })
        .collect()
    }

    fn find_max_steps_no_slope(&self) -> usize {
        // real input and the test has the first entrance
        let initial_state = State::new(Point::new(1, 1), Point::new(0, 1), Vec::new());
        let mut queue = VecDeque::new();
        // We've already made the first step
        queue.push_back((1, initial_state.clone()));

        let mut states_map: HashMap<State, usize> = HashMap::new();

        let mut max_steps = 0;

        while let Some((mut steps, state)) = queue.pop_front() {
            if self.reached_target(&state.curr) {
                max_steps = max_steps.max(steps);
                continue;
            }

            if let Some(saved_steps) = states_map.get_mut(&state) {
                if steps > *saved_steps {
                    *saved_steps = steps;
                } else {
                    continue;
                }
            }

            steps += 1;
            for p in self.get_next_moves_no_slope(&state) {
                let mut clone = state.clone();
                let next_point = match self.get_content(&p) {
                    b'^' | b'v' | b'<' | b'>' | b'.' => {
                        if clone.visited_slopes.contains(&p) {
                            continue;
                        }
                        clone.visited_slopes.push(p);

                        p
                    }
                    invalid => unreachable!("Invalid byte: '{invalid}'"),
                };

                clone.prev = clone.curr;
                clone.curr = next_point;
                queue.push_back((steps, clone));
            }
        }

        max_steps
    }
}

fn find_max_steps(input: &'static str) -> usize {
    let maze = Maze::from(input);

    maze.find_max_steps()
}

fn find_max_no_slope(input: &'static str) -> usize {
    let maze = Maze::from(input);

    maze.find_max_steps_no_slope()
}

fn part_1(input: &'static str) {
    let answer = find_max_steps(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &'static str) {
    let answer = find_max_no_slope(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    //TODO: uncomment the first input and remove the later when the solution is solved
    // let input = crate::utls::read_text_from_file("23", "23").leak();
    let input = crate::include_input!("23", "23");
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_solution() {
        assert_eq!(find_max_steps(INPUT), 94);
        assert_eq!(find_max_no_slope(INPUT), 154);
    }
}

