use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use itertools::Itertools;

use crate::utls::read_text_from_file;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn rotate(&self, rot: u8) -> Point {
        use Point as P;
        let Point { x, y, z } = *self;
        match rot {
            0 => P::new(x, y, z),
            1 => P::new(x, z, -y),
            2 => P::new(x, -y, -z),
            3 => P::new(x, -z, y),
            4 => P::new(y, x, -z),
            5 => P::new(y, z, x),
            6 => P::new(y, -x, z),
            7 => P::new(y, -z, -x),
            8 => P::new(z, x, y),
            9 => P::new(z, y, -x),
            10 => P::new(z, -x, -y),
            11 => P::new(z, -y, x),
            12 => P::new(-x, y, -z),
            13 => P::new(-x, z, y),
            14 => P::new(-x, -y, z),
            15 => P::new(-x, -z, -y),
            16 => P::new(-y, x, z),
            17 => P::new(-y, z, -x),
            18 => P::new(-y, -x, -z),
            19 => P::new(-y, -z, x),
            20 => P::new(-z, x, -y),
            21 => P::new(-z, y, x),
            22 => P::new(-z, -x, y),
            23 => P::new(-z, -y, -x),
            _ => unreachable!(),
        }
    }

    fn get_manhatten_dist(&self, rhs: &Point) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs() + (self.z - rhs.z).abs()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();

        Point { x, y, z }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}

impl Scanner {
    fn merge(&self, total_beacons: &mut HashSet<Point>) -> bool {
        for rot in 0..24 {
            let rotate_beacons: Vec<Point> = self.beacons.iter().map(|p| p.rotate(rot)).collect();
            let deltas: Vec<_> = total_beacons
                .iter()
                .cartesian_product(&rotate_beacons)
                .map(|(p1, p2)| *p1 - *p2)
                .collect();

            for delta_point in deltas {
                let translated = rotate_beacons.iter().map(|p| *p + delta_point);
                if translated
                    .clone()
                    .filter(|p| total_beacons.contains(p))
                    .count()
                    >= 12
                {
                    total_beacons.extend(translated);
                    return true;
                }
            }
        }

        false
    }

    fn merge_with_dis(&self, total_beacons: &mut HashSet<Point>) -> Option<Point> {
        for rot in 0..24 {
            let rotate_beacons: Vec<Point> = self.beacons.iter().map(|p| p.rotate(rot)).collect();
            let deltas: Vec<_> = total_beacons
                .iter()
                .cartesian_product(&rotate_beacons)
                .map(|(p1, p2)| *p1 - *p2)
                .collect();

            for delta_point in deltas {
                let translated = rotate_beacons.iter().map(|p| *p + delta_point);
                if translated
                    .clone()
                    .filter(|p| total_beacons.contains(p))
                    .count()
                    >= 12
                {
                    total_beacons.extend(translated);
                    return Some(delta_point);
                }
            }
        }

        None
    }
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        let beacons = value.lines().skip(1).map(Point::from).collect();

        Scanner { beacons }
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(Scanner::from).collect()
}

fn calc_beacons_count(input: &str) -> usize {
    let mut scanners = parse_input(input);
    let mut total_beacons: HashSet<_> = scanners.remove(0).beacons.into_iter().collect();
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if scanners[i].merge(&mut total_beacons) {
                scanners.swap_remove(i);
            }
        }
    }

    total_beacons.len()
}

fn calc_manhatten_dist(input: &str) -> i32 {
    let mut scanners = parse_input(input);
    let mut total_beacons: HashSet<_> = scanners.remove(0).beacons.into_iter().collect();
    let mut dists = Vec::new();
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(delta_p) = scanners[i].merge_with_dis(&mut total_beacons) {
                scanners.swap_remove(i);
                dists.push(delta_p);
            }
        }
    }

    dists
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.get_manhatten_dist(p2))
        .max()
        .unwrap()
}

fn part_1() {
    let input = read_text_from_file("21", "19");
    let answer = calc_beacons_count(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "19");
    let answer = calc_manhatten_dist(&input);

    println!("Part 2 answer is {answer}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_beacons_count(INPUT), 79);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_manhatten_dist(INPUT), 3621);
    }
}
