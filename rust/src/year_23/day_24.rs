#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i128,
    y: i128,
    z: i128,
}

impl Point {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }
}

impl From<&'static str> for Point {
    fn from(value: &'static str) -> Self {
        let mut nums = value.split(", ").map(|num| num.trim().parse().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        let z = nums.next().unwrap();

        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
struct LinearRepr {
    a: i128,
    b: i128,
    c: i128,
}

#[derive(Debug, Clone)]
struct Hail {
    position: Point,
    velocity: Point,
}

impl From<&'static str> for Hail {
    fn from(value: &'static str) -> Self {
        let (pos, ver) = value.split_once(" @ ").unwrap();
        let position = Point::from(pos);
        let velocity = Point::from(ver);

        Self { position, velocity }
    }
}

impl Hail {
    fn get_linear_repr(&self) -> LinearRepr {
        let a = self.velocity.y;
        let b = -self.velocity.x;
        let c = self.velocity.x * self.position.y - self.velocity.y * self.position.x;

        LinearRepr { a, b, c }
    }
}

fn intersection(line1: &LinearRepr, line2: &LinearRepr) -> Option<Point> {
    let divider = line1.a * line2.b - line2.a * line1.b;
    if divider == 0 {
        return None;
    }

    let x = (line1.b * line2.c - line2.b * line1.c) / divider;
    let y = (line1.c * line2.a - line2.c * line1.a) / divider;

    Some(Point::new(x, y, 0))
}

fn find_inter_count(input: &'static str, start: i128, end: i128) -> usize {
    let hails: Vec<_> = input.lines().map(Hail::from).collect();
    let exprs: Vec<_> = hails.iter().map(|hail| hail.get_linear_repr()).collect();

    let mut count = 0;
    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            let h1 = exprs[i];
            let h2 = exprs[j];

            if let Some(p) = intersection(&h1, &h2) {
                if i128::signum(p.x - hails[i].position.x) != i128::signum(hails[i].velocity.x) {
                    continue;
                }

                if i128::signum(p.x - hails[j].position.x) != i128::signum(hails[j].velocity.x) {
                    continue;
                }

                if i128::signum(p.y - hails[i].position.y) != i128::signum(hails[i].velocity.y) {
                    continue;
                }

                if i128::signum(p.y - hails[j].position.y) != i128::signum(hails[j].velocity.y) {
                    continue;
                }

                if p.x >= start && p.x <= end && p.y >= start && p.y <= end {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_1(input: &'static str) {
    let answer = find_inter_count(input, 200000000000000, 400000000000000);

    println!("Part 1 answer is {answer}");
}

fn part_2(_input: &'static str) {
    println!("Part 2 answer is solved using z3 solver in python. the Answer is 566373506408017");
}

pub fn run() {
    let input = crate::utls::read_text_from_file("23", "24").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_solution() {
        assert_eq!(find_inter_count(INPUT, 7, 27), 2);
    }
}
