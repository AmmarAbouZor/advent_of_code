use std::collections::{BTreeMap, VecDeque};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}

#[derive(Debug)]
struct Dir {
    files: Vec<File>,
}

const SEPARATOR: &str = "/";

impl Dir {
    fn new() -> Self {
        let files = Vec::new();

        Self { files }
    }

    fn get_files_size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum()
    }
}

fn get_path(parts: &VecDeque<&str>) -> String {
    let parts: Vec<_> = parts.iter().map(|p| p.to_owned()).collect();
    parts.join(SEPARATOR)
}

fn parse_input(input: &str) -> BTreeMap<String, Dir> {
    let mut current_path = VecDeque::new();
    current_path.push_back("");

    let mut dirs = BTreeMap::new();
    dirs.insert(get_path(&current_path), Dir::new());

    let mut pwd = get_path(&current_path);

    for line in input.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match (parts[0], parts[1]) {
            ("$", "cd") => match parts[2] {
                ".." => {
                    current_path.pop_back();
                    pwd = get_path(&current_path);
                }
                dir => {
                    current_path.push_back(dir);
                    pwd = get_path(&current_path);
                }
            },
            ("$", "ls") => {
                // no need to do any thing because we have only two commands
            }

            (p_1, p_2) => {
                if p_1 == "dir" {
                    let mut path_cloned = current_path.clone();
                    path_cloned.push_back(p_2);
                    dirs.insert(get_path(&path_cloned), Dir::new());
                } else {
                    let size = p_1.parse().unwrap();
                    let file = File::new(size);

                    let dir = dirs.get_mut(&pwd).unwrap();
                    dir.files.push(file);
                }
            }
        };
    }

    dirs
}

fn calc_sum_at_most(dirs: &BTreeMap<String, Dir>, size: usize) -> usize {
    let mut sum = 0;
    for dir in dirs.iter() {
        let dir_sum: usize = dirs
            .iter()
            .filter(|(path, _)| path.starts_with(dir.0))
            .map(|(_, d)| d.get_files_size())
            .sum();
        if dir_sum <= size {
            sum += dir_sum;
        }
    }

    sum
}

fn calc_smallest_needed(dirs: &BTreeMap<String, Dir>) -> usize {
    let total_size: usize = dirs.iter().map(|(_, dir)| dir.get_files_size()).sum();

    let availabe = 70000000 - total_size;

    let needed = 30000000 - availabe;

    let mut best = 0;
    let mut diff = usize::MAX;

    for dir in dirs.iter() {
        let dir_sum: usize = dirs
            .iter()
            .filter(|(path, _)| path.starts_with(dir.0))
            .map(|(_, d)| d.get_files_size())
            .sum();
        if dir_sum >= needed {
            let dir_diff = dir_sum - needed;
            if dir_diff < diff {
                diff = dir_diff;
                best = dir_sum;
            }
        }
    }

    best
}

fn part_1() {
    let input = read_text_from_file("22", "07");
    let dirs = parse_input(&input);
    let sum = calc_sum_at_most(&dirs, 100000);

    println!("Part 1 answer is {sum}");
}

fn part_2() {
    let input = read_text_from_file("22", "07");
    let dirs = parse_input(&input);
    let needed = calc_smallest_needed(&dirs);

    println!("Part 2 answer is {needed}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    #[test]
    fn test_part_1() {
        let dirs = parse_input(INPUT);

        assert_eq!(calc_sum_at_most(&dirs, 100000), 95437);
    }
}
