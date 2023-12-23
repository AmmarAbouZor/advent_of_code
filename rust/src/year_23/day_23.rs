use std::collections::{HashMap, HashSet, VecDeque};

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

    fn get_next_moves_no_slope(&self, point: &Point) -> Vec<Point> {
        let mut points = Vec::new();
        if point.row > 0 {
            points.push(Point::new(point.row - 1, point.col));
        }
        if point.row < self.cells.len() - 1 {
            points.push(Point::new(point.row + 1, point.col));
        }

        if point.col > 0 {
            points.push(Point::new(point.row, point.col - 1));
        }

        if point.col < self.cells[0].len() - 1 {
            points.push(Point::new(point.row, point.col + 1));
        }

        points
            .into_iter()
            .filter(|p| self.get_content(p) != b'#')
            .collect()
    }

    fn get_longest_dist_recr(
        start: &Point,
        end: &Point,
        map: &HashMap<Point, HashMap<Point, usize>>,
        visited: &HashSet<Point>,
    ) -> usize {
        if start == end {
            return 0;
        }

        let mut visited_clone = visited.clone();
        visited_clone.insert(*start);

        let mut distance = 0_usize;
        let start_node_map = map.get(start).unwrap();
        for neighbor in start_node_map.keys() {
            if !visited_clone.contains(neighbor) {
                let next_dist = Self::get_longest_dist_recr(neighbor, end, map, &visited_clone);
                distance = distance.max(next_dist + *start_node_map.get(neighbor).unwrap());
            }
        }

        distance
    }

    fn find_max_steps_graph(&self) -> usize {
        let start_pos = Point::new(0, 1);
        let end_pos = Point::new(self.cells.len() - 1, self.cells[0].len() - 2);

        // Define the node points so we can fill them in the graph below
        let mut node_points = vec![start_pos, end_pos];
        let nodes = (0..self.cells.len())
            .flat_map(|row| (0..self.cells[0].len()).map(move |col| Point::new(row, col)))
            .filter(|p| self.get_content(p) != b'#' && self.get_next_moves_no_slope(p).len() > 2);

        node_points.extend(nodes);

        // Fill the Graph with distances
        let mut graph: HashMap<Point, HashMap<Point, usize>> =
            node_points.iter().map(|p| (*p, HashMap::new())).collect();

        for node in &node_points {
            let mut queue: Vec<(Point, usize)> = vec![(*node, 0)];
            let mut visited: HashSet<Point> = HashSet::new();

            while let Some((curr_point, curr_dist)) = queue.pop() {
                if node_points.contains(&curr_point) && curr_dist != 0 {
                    graph
                        .entry(*node)
                        .or_default()
                        .entry(curr_point)
                        .or_insert(curr_dist);
                    continue;
                }

                for next_point in self.get_next_moves_no_slope(&curr_point) {
                    if visited.insert(next_point) {
                        queue.push((next_point, curr_dist + 1));
                    }
                }

                visited.insert(curr_point);
            }
        }

        // Use the graph to find the answer recursively
        Self::get_longest_dist_recr(&start_pos, &end_pos, &graph, &HashSet::new())
    }
}

fn find_max_steps(input: &'static str) -> usize {
    let maze = Maze::from(input);

    maze.find_max_steps()
}

fn find_max_no_slope(input: &'static str) -> usize {
    let maze = Maze::from(input);

    maze.find_max_steps_graph()
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
    let input = crate::utls::read_text_from_file("23", "23").leak();
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
