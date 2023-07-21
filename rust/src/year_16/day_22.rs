use std::collections::HashSet;

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    size: i32,
    used: i32,
    avail: i32,
}

impl Node {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let x_y: Vec<&str> = parts[0].split('-').collect();

        let x = x_y[1].trim_start_matches('x').parse().unwrap();
        let y = x_y[2].trim_start_matches('y').parse().unwrap();

        let size = parts[1].trim_end_matches('T').parse().unwrap();
        let used = parts[2].trim_end_matches('T').parse().unwrap();
        let avail = parts[3].trim_end_matches('T').parse().unwrap();

        Self {
            x,
            y,
            size,
            used,
            avail,
        }
    }

    fn get_symbol(&self, max_x: usize) -> char {
        match (self.x, self.y) {
            (0, 0) => 'S',
            (x, 0) if x == max_x => 'G',
            _ => {
                if self.used == 0 {
                    '_'
                } else if self.size > 100 {
                    '#'
                } else {
                    '.'
                }
            }
        }
    }
}

fn fetch_nodes() -> Vec<Node> {
    read_text_from_file("16", "22")
        .lines()
        .skip(2)
        .map(Node::from_line)
        .collect()
}

fn part_1() {
    let nodes = fetch_nodes();

    let mut hash = HashSet::new();
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && nodes[i].used.is_positive() && nodes[i].used < nodes[j].avail {
                hash.insert([(nodes[i].x, nodes[i].y), (nodes[j].x, nodes[j].y)]);
            }
        }
    }

    println!("viable pairs count is {}", hash.len());
}

fn part_2() {
    let nodes = fetch_nodes();
    let max_x = nodes.iter().map(|node| node.x).max().unwrap();
    let max_y = nodes.iter().map(|node| node.y).max().unwrap();

    let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];

    for node in nodes.iter() {
        grid[node.y][node.x] = node.get_symbol(max_x);
    }

    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }

    //    S....................................G
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ......................................
    //    ..............................########
    //    ......................................
    //    ..................................._..
    //    ......................................
    //    ......................................

    let empty_node = fetch_nodes()
        .into_iter()
        .find(|node| node.used == 0)
        .unwrap();
    let moves = 6 + empty_node.y + 8 + (max_x - 1) * 5;

    println!("moves count from grid: {moves}");
}

pub fn run() {
    part_1();
    part_2();
}
