#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrickType {
    AxisX,
    AxisY,
    AxisZ,
    Cube,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl From<&'static str> for Point {
    fn from(value: &'static str) -> Self {
        let mut nums = value.split(',').map(|num| num.parse().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        let z = nums.next().unwrap();

        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    end: Point,
}

impl From<&'static str> for Brick {
    fn from(value: &'static str) -> Self {
        let mut points = value.split('~').map(Point::from);
        let start = points.next().unwrap();
        let end = points.next().unwrap();

        Self { start, end }
    }
}

impl Brick {
    fn get_type(&self) -> BrickType {
        if self.start.x != self.end.x {
            BrickType::AxisX
        } else if self.start.y != self.end.y {
            BrickType::AxisY
        } else if self.start.z != self.end.z {
            BrickType::AxisZ
        } else {
            BrickType::Cube
        }
    }
}

fn get_bricks_count(input: &'static str) -> usize {
    let bricks: Vec<_> = input.lines().map(Brick::from).collect();
    dbg!(bricks);

    todo!()
}

fn part_1(input: &'static str) {}

fn part_2(input: &'static str) {}

pub fn run() {
    //TODO: uncomment the first input and remove the later when the solution is solved
    // let input = crate::utls::read_text_from_file("23", "22").leak();
    let input = crate::include_input!("23", "22");
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_solution() {
        assert_eq!(get_bricks_count(INPUT), 5);
    }
}

