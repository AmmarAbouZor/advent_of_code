use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn turn_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    command: Command,
    num: isize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let command = match line.chars().next().unwrap() {
            'N' => Command::North,
            'E' => Command::East,
            'S' => Command::South,
            'W' => Command::West,
            'L' => Command::Left,
            'R' => Command::Right,
            'F' => Command::Forward,
            invalid => unreachable!("invalid command input: '{invalid}'"),
        };

        let num = line[1..].parse().unwrap();

        Instruction { command, num }
    }
}

#[derive(Debug)]
struct StateDirection {
    east: isize,
    north: isize,
    direction: Direction,
}

impl Default for StateDirection {
    fn default() -> Self {
        StateDirection {
            east: 0,
            north: 0,
            direction: Direction::East,
        }
    }
}

impl StateDirection {
    fn apply_command(&mut self, instruction: Instruction) {
        match instruction.command {
            Command::North => self.north += instruction.num,
            Command::East => self.east += instruction.num,
            Command::South => self.north -= instruction.num,
            Command::West => self.east -= instruction.num,
            Command::Left => {
                let count = instruction.num / 90;
                for _ in 0..count {
                    self.direction.turn_left();
                }
            }
            Command::Right => {
                let count = instruction.num / 90;
                for _ in 0..count {
                    self.direction.turn_right();
                }
            }
            Command::Forward => match self.direction {
                Direction::North => self.north += instruction.num,
                Direction::East => self.east += instruction.num,
                Direction::South => self.north -= instruction.num,
                Direction::West => self.east -= instruction.num,
            },
        }
    }

    fn get_manhatten_distance(&self) -> isize {
        self.east.abs() + self.north.abs()
    }
}

fn calc_distance_direction(input: &str) -> isize {
    let mut state = StateDirection::default();
    input.lines().map(Instruction::from).for_each(|inst| {
        state.apply_command(inst);
    });

    state.get_manhatten_distance()
}

#[derive(Debug)]
struct StateWaypoint {
    pos_x: isize,
    pos_y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
}

impl Default for StateWaypoint {
    fn default() -> Self {
        StateWaypoint {
            pos_x: 0,
            pos_y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }
}

impl StateWaypoint {
    fn apply_command(&mut self, instruction: Instruction) {
        match instruction.command {
            Command::North => self.waypoint_y += instruction.num,
            Command::East => self.waypoint_x += instruction.num,
            Command::South => self.waypoint_y -= instruction.num,
            Command::West => self.waypoint_x -= instruction.num,
            Command::Left => {
                let count = instruction.num / 90;
                for _ in 0..count {
                    self.turn_left();
                }
            }
            Command::Right => {
                let count = instruction.num / 90;
                for _ in 0..count {
                    self.turn_right();
                }
            }
            Command::Forward => {
                self.pos_x += self.waypoint_x * instruction.num;
                self.pos_y += self.waypoint_y * instruction.num;
            }
        }
    }

    fn turn_right(&mut self) {
        match (self.waypoint_x >= 0, self.waypoint_y >= 0) {
            // 10, 1
            (true, true) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_y *= -1;
            }
            // 1, -10
            (true, false) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_y *= -1;
            }
            // -10, -1
            (false, false) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_y *= -1;
            }
            // -1 , 10
            (false, true) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_y *= -1;
            }
        }
    }

    fn turn_left(&mut self) {
        match (self.waypoint_x >= 0, self.waypoint_y >= 0) {
            (true, true) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_x *= -1;
            }
            (true, false) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_x *= -1;
            }
            (false, false) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_x *= -1;
            }
            (false, true) => {
                std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
                self.waypoint_x *= -1;
            }
        }
    }

    fn get_manhatten_distance(&self) -> isize {
        self.pos_x.abs() + self.pos_y.abs()
    }
}

fn calc_distance_waypoint(input: &str) -> isize {
    let mut state = StateWaypoint::default();
    input.lines().map(Instruction::from).for_each(|inst| {
        state.apply_command(inst);
    });

    state.get_manhatten_distance()
}

fn part_1(input: &str) {
    let answer = calc_distance_direction(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_distance_waypoint(input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("20", "12");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_distance_direction(INPUT), 25);
        assert_eq!(calc_distance_waypoint(INPUT), 286);
    }
}

