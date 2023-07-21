use crate::utls::{get_input_path, read_lines_from_file};

#[derive(Debug)]
struct TriangleSides {
    a: u16,
    b: u16,
    c: u16,
}

impl From<&str> for TriangleSides {
    fn from(line: &str) -> Self {
        let mut parts = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let c = parts.next().unwrap();

        TriangleSides { a, b, c }
    }
}

impl TriangleSides {
    fn new(a: u16, b: u16, c: u16) -> Self {
        Self { a, b, c }
    }

    fn is_triangle(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

fn part_1() {
    let triangles = read_lines_from_file(&get_input_path("16", "03"))
        .iter()
        .map(|line| TriangleSides::from(line.as_str()))
        .filter(|tr| tr.is_triangle())
        .count();

    println!("num of triangles is {triangles}");
}

fn read_nums_vertically() -> Vec<u16> {
    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut v3 = vec![];

    for line in read_lines_from_file(&get_input_path("16", "03")) {
        let temp = TriangleSides::from(line.as_str());
        v1.push(temp.a);
        v2.push(temp.b);
        v3.push(temp.c);
    }

    v1.append(&mut v2);
    v1.append(&mut v3);

    v1
}

fn part_2() {
    let nums = read_nums_vertically();

    let count = (0..nums.len())
        .step_by(3)
        .map(|i| TriangleSides::new(nums[i], nums[i + 1], nums[i + 2]))
        .filter(|tr| tr.is_triangle())
        .count();

    println!("part_2: count is {count}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_triangle() {
        let tr = TriangleSides::from("5 10 25");
        assert!(!tr.is_triangle());
    }
}
