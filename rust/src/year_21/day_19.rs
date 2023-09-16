use std::collections::HashSet;

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

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    rotate_count: u8,
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        let beacons = value.lines().skip(1).map(Point::from).collect();

        Scanner {
            beacons,
            rotate_count: 0,
        }
    }
}

impl Scanner {
    fn rotate(&mut self) -> bool {
        use std::mem::swap;
        match self.rotate_count {
            0 => {
                for point in self.beacons.iter_mut() {
                    point.x *= -1;
                    swap(&mut point.y, &mut point.z);
                    point.y *= -1;
                    point.z *= -1;
                }
                self.rotate_count += 1;
                return true;
            }
            1 => {
                for point in self.beacons.iter_mut() {
                    swap(&mut point.x, &mut point.y);
                    swap(&mut point.y, &mut point.z);
                    point.y *= -1;
                    point.z *= -1;
                }
                self.rotate_count += 1;
                return true;
            }
            2 => {
                for point in self.beacons.iter_mut() {
                    point.x *= -1;
                    point.y *= -1;
                }
                self.rotate_count += 1;
                return true;
            }
            3 => {
                for point in self.beacons.iter_mut() {
                    swap(&mut point.x, &mut point.y);
                    point.z *= -1;
                }
                self.rotate_count += 1;
                return true;
            }
            _ => false,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Scanner> + '_ {
    input.split("\n\n").map(Scanner::from)
}

fn calc_beacons_count(input: &str) -> usize {
    let mut scanners = parse_input(input);
    let first_scanner = scanners.next().unwrap();
    let mut intersect_set: HashSet<_> = HashSet::from_iter(first_scanner.beacons.into_iter());
    for mut scanner in scanners {
        loop {
            let intersect_len = scanner
                .beacons
                .iter()
                .filter(|b| intersect_set.contains(b))
                .count();
            if intersect_len < 12 {
                assert!(scanner.rotate());
            } else {
                for beacon in scanner.beacons {
                    intersect_set.insert(beacon);
                }
                break;
            }
        }
    }

    intersect_set.len()
}

fn part_1() {
    let input = read_text_from_file("21", "19");
    let answer = calc_beacons_count(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {}

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
    fn test_rotate() {
        const SCANNER: &str = r"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7";
        let mut scanner = Scanner::from(SCANNER);
        while scanner.rotate() {}
        assert_eq!(scanner.rotate_count, 4);
        assert_eq!(scanner.beacons.last().unwrap(), &Point::new(0, 7, -8));
        assert_eq!(scanner.beacons[4], Point::new(-6, -4, -5));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(calc_beacons_count(INPUT), 79);
    }
}
