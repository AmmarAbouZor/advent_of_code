use std::{collections::HashSet, ops::RangeInclusive};

use crate::utls::read_text_from_file;

type Cubes = HashSet<(i32, i32, i32)>;

#[derive(Debug)]
struct Step {
    on: bool,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl From<&str> for Step {
    fn from(line: &str) -> Self {
        fn parse_range(rng: &str) -> RangeInclusive<i32> {
            let (start, end) = rng.split_once("..").unwrap();

            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        }

        let (on, rest) = line.split_once(' ').unwrap();
        let on = on == "on";
        let mut nums = rest.split(',').map(|part| &part[2..]);
        let x = parse_range(nums.next().unwrap());
        let y = parse_range(nums.next().unwrap());
        let z = parse_range(nums.next().unwrap());

        Step { on, x, y, z }
    }
}

impl Step {
    fn apply(&self, lid_set: &mut Cubes, limit: &RangeInclusive<i32>) {
        if self.on {
            let action = |x, y, z, set: &mut Cubes| {
                set.insert((x, y, z));
            };

            self.apply_intern(lid_set, limit, action)
        } else {
            let action = |x, y, z, set: &mut Cubes| {
                set.remove(&(x, y, z));
            };

            self.apply_intern(lid_set, limit, action)
        };
    }

    fn apply_intern<A: FnMut(i32, i32, i32, &mut Cubes)>(
        &self,
        lid_set: &mut Cubes,
        limit: &RangeInclusive<i32>,
        mut action: A,
    ) {
        for x in self.x.clone().filter(|x| limit.contains(x)) {
            for y in self.y.clone().filter(|y| limit.contains(y)) {
                for z in self.z.clone().filter(|z| limit.contains(z)) {
                    action(x, y, z, lid_set);
                }
            }
        }
    }
}

fn calc_lid_cubes(input: &str) -> usize {
    let steps: Vec<Step> = input.lines().map(Step::from).collect();

    let mut lid_set = HashSet::new();

    let limit = RangeInclusive::new(-50, 50);

    for step in steps {
        step.apply(&mut lid_set, &limit);
    }

    lid_set.len()
}

fn part_1() {
    let input = read_text_from_file("21", "22");
    let answer = calc_lid_cubes(input.as_str());

    println!("Part 1 answer is {answer}")
}

fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_part_1() {
        assert_eq!(calc_lid_cubes(INPUT), 590784)
    }
}

