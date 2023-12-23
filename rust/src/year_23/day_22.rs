use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrickType {
    AxisX,
    AxisY,
    AxisZ,
    Cube,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.start.x, self.start.y, self.start.z, self.end.x, self.end.y, self.end.z
        )
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

    fn min_z(&self) -> usize {
        self.start.z.min(self.end.z)
    }

    fn max_z(&self) -> usize {
        self.start.z.max(self.end.z)
    }

    fn get_cubes(&self) -> Vec<Point> {
        match self.get_type() {
            BrickType::AxisX => (self.start.x..=self.end.x)
                .map(|x| Point::new(x, self.start.y, self.start.z))
                .collect(),
            BrickType::AxisY => (self.start.y..=self.end.y)
                .map(|y| Point::new(self.start.x, y, self.start.z))
                .collect(),
            BrickType::AxisZ => (self.start.z..=self.end.z)
                .map(|z| Point::new(self.start.x, self.start.y, z))
                .collect(),
            BrickType::Cube => vec![self.start],
        }
    }
}

fn finish_fall(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|b| b.min_z());

    for i in 0..bricks.len() {
        let settled_bricks: HashSet<Point> =
            bricks[..i].iter().flat_map(|b| b.get_cubes()).collect();
        while bricks[i].min_z() > 1 {
            let mut clone = bricks[i].clone();
            clone.start.z -= 1;
            clone.end.z -= 1;
            if clone
                .get_cubes()
                .into_iter()
                .any(|p| settled_bricks.contains(&p))
            {
                break;
            }
            bricks[i] = clone;
        }
    }
}

fn get_dependencies(bricks: &[Brick]) -> Vec<Vec<usize>> {
    let mut dependencies = vec![vec![]; bricks.len()];

    for (idx, brick) in bricks.iter().enumerate() {
        let max_z = brick.max_z();
        let cubes = brick.get_cubes();
        bricks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.min_z() == max_z + 1)
            .for_each(|(d_idx, d_brick)| {
                let mut clone = d_brick.clone();
                clone.start.z -= 1;
                clone.end.z -= 1;
                if clone.get_cubes().iter().any(|c| cubes.contains(c)) {
                    dependencies[idx].push(d_idx);
                }
            });
    }

    dependencies
}

fn get_bricks_count(input: &'static str) -> usize {
    let mut bricks: Vec<_> = input.lines().map(Brick::from).collect();

    finish_fall(&mut bricks);

    let dependencies = get_dependencies(&bricks);

    let mut dep_count = BTreeMap::new();

    dependencies.iter().flatten().for_each(|&num| {
        dep_count
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    dependencies
        .iter()
        .filter(|dep| dep.iter().all(|num| dep_count[num] > 1))
        .count()
}

fn get_fall_sum(input: &'static str) -> usize {
    let mut bricks: Vec<_> = input.lines().map(Brick::from).collect();

    finish_fall(&mut bricks);

    let dependencies = get_dependencies(&bricks);

    let mut dep_count = BTreeMap::new();

    dependencies.iter().flatten().for_each(|&num| {
        dep_count
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let mut sum = 0;

    for dep in dependencies.iter() {
        let mut count_clone = dep_count.clone();
        let mut fallen_stack = Vec::new();
        for b_id in dep.iter() {
            let count = count_clone.get_mut(b_id).unwrap();
            *count -= 1;
            if *count == 0 {
                fallen_stack.push(b_id);
            }
        }

        while let Some(fall_id) = fallen_stack.pop() {
            sum += 1;
            for b_id in dependencies[*fall_id].iter() {
                let count = count_clone.get_mut(b_id).unwrap();
                *count -= 1;
                if *count == 0 {
                    fallen_stack.push(b_id);
                }
            }
        }
    }

    sum
}

fn part_1(input: &'static str) {
    let answer = get_bricks_count(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &'static str) {
    let answer = get_fall_sum(input);

    println!("Part 2 answer is {answer}");
}

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
        assert_eq!(get_fall_sum(INPUT), 7);
    }
}
