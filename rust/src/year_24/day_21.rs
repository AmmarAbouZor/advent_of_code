use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Dir {
    A,
    Up,
    Right,
    Down,
    Left,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Dir::A => 'A',
            Dir::Up => '^',
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
        };

        write!(f, "{char}")
    }
}

impl Dir {
    fn go_to(self, target: Dir) -> &'static [&'static [Dir]] {
        use Dir::*;
        match self {
            A => match target {
                A => &[&[A]],
                Up => &[&[Left, A]],
                Right => &[&[Down, A]],
                Down => &[&[Left, Down, A], &[Down, Left, A]],
                Left => &[&[Down, Left, Left, A]],
            },
            Up => match target {
                A => &[&[Right, A]],
                Up => &[&[A]],
                Right => &[&[Right, Down, A], &[Down, Right, A]],
                Down => &[&[Down, A]],
                Left => &[&[Down, Left, A]],
            },
            Right => match target {
                A => &[&[Up, A]],
                Up => &[&[Up, Left, A], &[Left, Up, A]],
                Right => &[&[A]],
                Down => &[&[Left, A]],
                Left => &[&[Left, Left, A]],
            },
            Down => match target {
                A => &[&[Up, Right, A], &[Right, Up, A]],
                Up => &[&[Up, A]],
                Right => &[&[Right, A]],
                Down => &[&[A]],
                Left => &[&[Left, A]],
            },
            Left => match target {
                A => &[&[Right, Right, Up, A]],
                Up => &[&[Right, Up, A]],
                Right => &[&[Right, Right, A]],
                Down => &[&[Right, A]],
                Left => &[&[A]],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Num {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<char> for Num {
    fn from(value: char) -> Self {
        use Num::*;
        match value {
            'A' => A,
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            invalid => panic!("{invalid}"),
        }
    }
}

impl Num {
    fn go_to(self, target: Num) -> &'static [&'static [Dir]] {
        use Dir::*;
        use Num as N;
        match self {
            N::A => match target {
                N::A => &[&[A]],
                N::Zero => &[&[Left, A]],
                N::One => &[&[Up, Left, Left, A]],
                N::Two => &[&[Up, Left, A], &[Left, Up, A]],
                N::Three => &[&[Up, A]],
                N::Four => &[&[Up, Up, Left, Left, A]],
                N::Five => &[&[Up, Up, Left, A], &[Left, Up, Up, A]],
                N::Six => &[&[Up, Up, A]],
                N::Seven => &[&[Up, Up, Up, Left, Left, A]],
                N::Eight => &[&[Up, Up, Up, Left, A], &[Left, Up, Up, Up, A]],
                N::Nine => &[&[Up, Up, Up, A]],
            },
            N::Zero => match target {
                N::A => &[&[Right, A]],
                N::Zero => &[&[A]],
                N::One => &[&[Up, Left, A]],
                N::Two => &[&[Up, A]],
                N::Three => &[&[Up, Right, A]],
                N::Four => &[&[Up, Up, Left, A]],
                N::Five => &[&[Up, Up, A]],
                N::Six => &[&[Up, Up, Right, A]],
                N::Seven => &[&[Up, Up, Up, Left, A]],
                N::Eight => &[&[Up, Up, Up, A]],
                N::Nine => &[&[Up, Up, Up, Right, A]],
            },
            N::One => match target {
                N::A => &[&[Right, Right, Down, A]],
                N::Zero => &[&[Right, Down, A]],
                N::One => &[&[A]],
                N::Two => &[&[Right, A]],
                N::Three => &[&[Right, Right, A]],
                N::Four => &[&[Up, A]],
                N::Five => &[&[Up, Right, A], &[Right, Up, A]],
                N::Six => &[&[Up, Right, Right, A], &[Right, Right, Up, A]],
                N::Seven => &[&[Up, Up, A]],
                N::Eight => &[&[Up, Up, Right, A], &[Right, Up, Up, A]],
                N::Nine => &[&[Up, Up, Right, Right, A], &[Right, Right, Up, Up, A]],
            },
            N::Two => match target {
                N::A => &[&[Right, Down, A], &[Down, Right, A]],
                N::Zero => &[&[Down, A]],
                N::One => &[&[Left, A]],
                N::Two => &[&[A]],
                N::Three => &[&[Right, A]],
                N::Four => &[&[Up, Left, A], &[Left, Up, A]],
                N::Five => &[&[Up, A]],
                N::Six => &[&[Up, Right, A], &[Right, Up, A]],
                N::Seven => &[&[Up, Up, Left, A], &[Left, Up, Up, A]],
                N::Eight => &[&[Up, Up, A]],
                N::Nine => &[&[Up, Up, Right, A], &[Right, Up, Up, A]],
            },
            N::Three => match target {
                N::A => &[&[Down, A]],
                N::Zero => &[&[Left, Down, A], &[Down, Left, A]],
                N::One => &[&[Left, Left, A]],
                N::Two => &[&[Left, A]],
                N::Three => &[&[A]],
                N::Four => &[&[Left, Left, Up, A], &[Up, Left, Left, A]],
                N::Five => &[&[Left, Up, A], &[Up, Left, A]],
                N::Six => &[&[Up, A]],
                N::Seven => &[&[Up, Up, Left, Left, A], &[Left, Left, Up, Up, A]],
                N::Eight => &[&[Up, Up, Left, A], &[Left, Up, Up, A]],
                N::Nine => &[&[Up, Up, A]],
            },
            N::Four => match target {
                N::A => &[&[Right, Right, Down, Down, A]],
                N::Zero => &[&[Right, Down, Down, A]],
                N::One => &[&[Down, A]],
                N::Two => &[&[Right, Down, A], &[Down, Right, A]],
                N::Three => &[&[Right, Right, Down, A], &[Down, Right, Right, A]],
                N::Four => &[&[A]],
                N::Five => &[&[Right, A]],
                N::Six => &[&[Right, Right, A]],
                N::Seven => &[&[Up, A]],
                N::Eight => &[&[Up, Right, A], &[Right, Up, A]],
                N::Nine => &[&[Up, Right, Right, A], &[Right, Right, Up, A]],
            },
            N::Five => match target {
                N::A => &[&[Right, Down, Down, A], &[Down, Down, Right, A]],
                N::Zero => &[&[Down, Down, A]],
                N::One => &[&[Left, Down, A], &[Down, Left, A]],
                N::Two => &[&[Down, A]],
                N::Three => &[&[Right, Down, A], &[Down, Right, A]],
                N::Four => &[&[Left, A]],
                N::Five => &[&[A]],
                N::Six => &[&[Right, A]],
                N::Seven => &[&[Up, Left, A], &[Left, Up, A]],
                N::Eight => &[&[Up, A]],
                N::Nine => &[&[Up, Right, A], &[Right, Up, A]],
            },
            N::Six => match target {
                N::A => &[&[Down, Down, A]],
                N::Zero => &[&[Down, Down, Left, A], &[Left, Down, Down, A]],
                N::One => &[&[Left, Left, Down, A], &[Down, Left, Left, A]],
                N::Two => &[&[Left, Down, A], &[Down, Left, A]],
                N::Three => &[&[Down, A]],
                N::Four => &[&[Left, Left, A]],
                N::Five => &[&[Left, A]],
                N::Six => &[&[A]],
                N::Seven => &[&[Left, Left, Up, A], &[Up, Left, Left, A]],
                N::Eight => &[&[Left, Up, A], &[Up, Left, A]],
                N::Nine => &[&[Up, A]],
            },
            N::Seven => match target {
                N::A => &[&[Right, Right, Down, Down, Down, A]],
                N::Zero => &[&[Right, Right, Down, Down, Down, A]],
                N::One => &[&[Down, Down, A]],
                N::Two => &[&[Right, Down, Down, A], &[Down, Down, Right, A]],
                N::Three => &[
                    &[Right, Right, Down, Down, A],
                    &[Down, Down, Right, Right, A],
                ],
                N::Four => &[&[Down, A]],
                N::Five => &[&[Right, Down, A], &[Down, Right, A]],
                N::Six => &[&[Right, Right, Down, A], &[Down, Right, Right, A]],
                N::Seven => &[&[A]],
                N::Eight => &[&[Right, A]],
                N::Nine => &[&[Right, Right, A]],
            },
            N::Eight => match target {
                N::A => &[&[Down, Down, Down, Right, A], &[Right, Down, Down, Down, A]],
                N::Zero => &[&[Down, Down, Down, A]],
                N::One => &[&[Left, Down, Down, A], &[Down, Down, Left, A]],
                N::Two => &[&[Down, Down, A]],
                N::Three => &[&[Right, Down, Down, A], &[Down, Down, Right, A]],
                N::Four => &[&[Left, Down, A], &[Down, Left, A]],
                N::Five => &[&[Down, A]],
                N::Six => &[&[Right, Down, A], &[Down, Right, A]],
                N::Seven => &[&[Left, A]],
                N::Eight => &[&[A]],
                N::Nine => &[&[Right, A]],
            },
            N::Nine => match target {
                N::A => &[&[Down, Down, Down, A]],
                N::Zero => &[&[Left, Down, Down, Down, A], &[Down, Down, Down, Left, A]],
                N::One => &[&[Left, Left, Down, Down, A], &[Down, Down, Left, Left, A]],
                N::Two => &[&[Left, Down, Down, A], &[Down, Down, Left, A]],
                N::Three => &[&[Down, Down, A]],
                N::Four => &[&[Left, Left, Down, A], &[Down, Left, Left, A]],
                N::Five => &[&[Left, Down, A], &[Down, Left, A]],
                N::Six => &[&[Down, A]],
                N::Seven => &[&[Left, Left, A]],
                N::Eight => &[&[Left, A]],
                N::Nine => &[&[A]],
            },
        }
    }
}

fn parse(input: &str) -> Vec<(usize, Vec<Num>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let number = line[..3].parse().unwrap();
            let items = line.chars().map(Num::from).collect();
            (number, items)
        })
        .collect()
}

fn print_dirs(dirs: &[Dir]) {
    for d in dirs {
        print!("{d}");
    }
    println!();
}

fn get_combinations(nums: &[Num]) -> Vec<Vec<Dir>> {
    let mut n = vec![Num::A];
    n.extend(nums);
    let nums = n;

    let mut res: Vec<Vec<Dir>> = vec![Vec::new()];
    for win in nums.windows(2) {
        let n1 = win[0];
        let n2 = win[1];
        let mut new_comb = Vec::new();
        let ways = n1.go_to(n2);
        for com in &res {
            for way in ways {
                let mut clone = com.clone();
                clone.extend_from_slice(way);
                new_comb.push(clone);
            }
        }

        res = new_comb;
    }

    res
}

fn get_combs_dirs(input: &[Dir]) -> Vec<Vec<Dir>> {
    let mut n = vec![Dir::A];
    n.extend(input);
    let input = n;

    let mut res: Vec<Vec<Dir>> = vec![Vec::new()];
    for win in input.windows(2) {
        let d1 = win[0];
        let d2 = win[1];
        let mut new_comb = Vec::new();
        let ways = d1.go_to(d2);
        for com in &res {
            for way in ways {
                let mut clone = com.clone();
                clone.extend_from_slice(way);
                new_comb.push(clone);
            }
        }

        res = new_comb;
    }

    res
}

fn calc_score(items: &[Num], factor: usize) -> usize {
    let mut min = usize::MAX;
    let combs = get_combinations(items);
    for c in combs {
        // print_dirs(&c);
        let nested = get_combs_dirs(&c);
        for n in nested {
            // print_dirs(&n);
            let nested_2 = get_combs_dirs(&n);
            for n2 in nested_2 {
                // print_dirs(&n2);
                min = min.min(n2.len());
            }

            // println!("--------------------------------------------")
        }
        // println!("--------------------------------------------")
    }

    min * factor
}

fn calc_all_score(input: &str) -> usize {
    let nums = parse(input);
    let res: usize = nums.iter().map(|(d, items)| calc_score(items, *d)).sum();
    res
}

fn part_1(input: &'static str) {
    let res = calc_all_score(input);
    println!("Part 1 result is {res}");
}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "21").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
029A
980A
179A
456A
379A";

    #[test]
    fn test_solution() {
        let res = calc_all_score(INPUT);
        assert_eq!(res, 126384);
    }
}
