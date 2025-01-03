use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

use crate::utls::read_text_from_file;

type Cubes = HashSet<(i32, i32, i32)>;

#[derive(Debug, Clone)]
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
    fn new(
        on: bool,
        x: RangeInclusive<i32>,
        y: RangeInclusive<i32>,
        z: RangeInclusive<i32>,
    ) -> Self {
        Self { on, x, y, z }
    }

    // Used for the naive solution
    fn apply_limit(&self, lid_set: &mut Cubes, limit: &RangeInclusive<i32>) {
        if self.on {
            let action = |x, y, z, set: &mut Cubes| {
                set.insert((x, y, z));
            };

            self.apply_intern_limit(lid_set, limit, action)
        } else {
            let action = |x, y, z, set: &mut Cubes| {
                set.remove(&(x, y, z));
            };

            self.apply_intern_limit(lid_set, limit, action)
        };
    }

    // Used for the naive solution
    fn apply_intern_limit<A: FnMut(i32, i32, i32, &mut Cubes)>(
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

    // The efficient solution
    fn combine_sub(&self, other: &Step) -> Option<Step> {
        let rng_x = combine_range(&self.x, &other.x)?;
        let rng_y = combine_range(&self.y, &other.y)?;
        let rng_z = combine_range(&self.z, &other.z)?;

        Some(Step::new(self.on, rng_x, rng_y, rng_z))
    }

    // The efficient solution
    fn calc_volume(&self) -> isize {
        (*self.x.end() - *self.x.start() + 1) as isize
            * (*self.y.end() - *self.y.start() + 1) as isize
            * (*self.z.end() - *self.z.start() + 1) as isize
    }
}

#[inline]
fn combine_range(
    rng: &RangeInclusive<i32>,
    other: &RangeInclusive<i32>,
) -> Option<RangeInclusive<i32>> {
    if rng.end() < other.start() || rng.start() > other.end() {
        return None;
    }

    let start = min(max(rng.start(), other.start()), other.end());
    let end = min(max(rng.end(), other.start()), other.end());

    Some(RangeInclusive::new(*start, *end))
}

// The limit here is small enough to apply the naive solution here
fn calc_lid_cubes_limit(input: &str) -> usize {
    let steps: Vec<Step> = input.lines().map(Step::from).collect();

    let mut lid_set = HashSet::new();

    let limit = RangeInclusive::new(-50, 50);

    for step in steps {
        step.apply_limit(&mut lid_set, &limit);
    }

    lid_set.len()
}

fn part_1() {
    let input = read_text_from_file("21", "22");
    let answer = calc_lid_cubes_limit(input.as_str());

    println!("Part 1 answer is {answer}")
}

fn part_2() {
    let input = read_text_from_file("21", "22");
    let answer = calc_all_lid_cubes(input.as_str());

    println!("Part 2 answer is {answer}")
}

fn calc_all_lid_cubes(input: &str) -> isize {
    let steps: Vec<Step> = input.lines().map(Step::from).collect();

    (0..steps.len())
        .filter(|&i| steps[i].on)
        .map(|i| calc_sub_step(&steps[i], &steps[i + 1..]))
        .sum::<isize>()
}

fn calc_sub_step(step: &Step, rest: &[Step]) -> isize {
    let rngs_to_sub: Vec<Step> = rest.iter().filter_map(|s| s.combine_sub(step)).collect();

    let rngs_sub_sum: isize = (0..rngs_to_sub.len())
        .map(|i| calc_sub_step(&rngs_to_sub[i], &rngs_to_sub[i + 1..]))
        .sum();

    step.calc_volume() - rngs_sub_sum
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
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

        assert_eq!(calc_lid_cubes_limit(INPUT), 590784)
    }

    #[test]
    fn test_part_2() {
        const INPUT: &str = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        assert_eq!(calc_all_lid_cubes(INPUT), 2758514936282235)
    }
}
