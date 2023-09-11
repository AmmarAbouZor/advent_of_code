use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy)]
struct SnailNum {
    value: u32,
    depth: u32,
    offset: usize,
}

impl SnailNum {
    fn new(value: u32, depth: u32, offset: usize) -> Self {
        Self {
            value,
            depth,
            offset,
        }
    }
}

fn line_to_nums(line: &str) -> Vec<SnailNum> {
    let mut depth = 0;
    let mut nums = Vec::new();
    let mut offset = 0;
    for ch in line.chars() {
        match ch {
            '[' => {
                depth += 1;
                offset += 1;
            }
            ']' => {
                depth -= 1;
                offset += 1
            }
            ',' => {}
            ch => {
                let num = ch.to_digit(10).unwrap();
                nums.push(SnailNum::new(num, depth, offset));
                offset = 0;
            }
        }
    }
    assert!(depth == 0);

    nums
}

#[inline]
fn get_explode_pos(nums: &Vec<SnailNum>) -> Option<(usize, usize)> {
    assert!(nums.iter().all(|num| num.depth < 6));
    if let Some((idx, _)) = nums.iter().enumerate().find(|(_, &num)| num.depth == 5) {
        // Make sure the nesting is maximum 5 levels only
        assert_eq!(nums[idx + 1].depth, 5);
        // Make sure the return values are a pair
        assert_eq!(nums[idx + 1].offset, 0);

        Some((idx, idx + 1))
    } else {
        return None;
    }
}

#[inline]
fn get_split_pos(nums: &Vec<SnailNum>) -> Option<usize> {
    nums.iter()
        .enumerate()
        .find(|(_, &num)| num.value > 9)
        .map(|(idx, _)| idx)
}

#[inline]
fn add_snails(mut s_1: Vec<SnailNum>, mut s_2: Vec<SnailNum>) -> Vec<SnailNum> {
    let first_s1 = s_1.first_mut().unwrap();
    first_s1.offset += 1;
    let last_s1 = s_1.last().unwrap();
    let first_s2 = s_2.first_mut().unwrap();
    first_s2.offset += last_s1.depth as usize;

    s_1.append(&mut s_2);

    s_1.iter_mut().for_each(|num| num.depth += 1);

    let mut nums = s_1;

    let mut changed = true;
    while changed {
        // dbg!(&nums);
        changed = false;
        // ==== Explode ====
        if let Some((idx_left, idx_right)) = get_explode_pos(&nums) {
            // println!("Explode: left {idx_left}, right: {idx_right}");
            changed = true;
            let mut idx_to_remove = Vec::new();
            // Left side
            if idx_left == 0 {
                nums[idx_left].depth -= 1;
                nums[idx_left].value = 0;
                nums[idx_left].offset -= 1;
            } else {
                let current_val = nums[idx_left].value;
                let current_offset = nums[idx_left].offset;
                let prev_left = &mut nums[idx_left - 1];
                if current_offset == 1 {
                    prev_left.value += current_val;
                    idx_to_remove.push(idx_left);
                } else {
                    prev_left.value += current_val;
                    nums[idx_left].depth -= 1;
                    nums[idx_left].value = 0;
                    nums[idx_left].offset -= 1;
                }
            }

            // Right side
            if idx_right == nums.len() - 1 {
                nums[idx_right].depth -= 1;
                nums[idx_right].value = 0;
            } else {
                let current_value = nums[idx_right].value;
                let next_right = &mut nums[idx_right + 1];
                if next_right.offset == 1 {
                    next_right.value += current_value;
                    next_right.offset = 0;
                    idx_to_remove.push(idx_right);
                } else {
                    next_right.value += current_value;
                    next_right.offset -= 1;
                    nums[idx_right].depth -= 1;
                    nums[idx_right].value = 0;
                }
            }

            // Remove items
            // Start at the right values
            for &idx in idx_to_remove.iter().rev() {
                // println!("Remove {idx}");
                nums.remove(idx);
            }
        }
        // ==== Split ====
        else if let Some(split_idx) = get_split_pos(&nums) {
            // println!("Splite: {split_idx}");
            changed = true;
            let num = nums[split_idx].value;
            nums[split_idx].depth += 1;
            nums[split_idx].value = num / 2;
            nums[split_idx].offset += 1;

            let new_num = SnailNum::new(num - nums[split_idx].value, nums[split_idx].depth, 0);
            nums.insert(split_idx + 1, new_num);

            if let Some(next_next) = nums.iter_mut().nth(split_idx + 2) {
                next_next.offset += 1;
            }
        }
    }

    nums
}

fn calc_magnitude(input: &str) -> usize {
    let mut snails = input.lines().map(line_to_nums);
    let mut current_val = snails.next().unwrap();
    for mut next_snail in snails {
        current_val = add_snails(current_val, next_snail);
        dbg!(&current_val);
        panic!("enough");
    }

    dbg!(current_val);

    todo!()
}

fn part_1() {
    let input = read_text_from_file("21", "18");
    let answer = calc_magnitude(input.as_str());

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

    const BASIC_INPUT: &str = r"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";

    const INPUT_1: &str = r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    const INPUT_2: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    // #[test]
    // fn test_explode() {
    //     let input = [
    //         "[[[[[9,8],1],2],3],4]",
    //         "[7,[6,[5,[4,[3,2]]]]]",
    //         "[[6,[5,[4,[3,2]]]],1]",
    //         "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    //         "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    //     ];
    //     for line in input {
    //         println!("{line}");
    //         let mut nums = line_to_nums(line);
    //
    //         let (idx_left, idx_right) = get_explode_pos(&nums).unwrap();
    //         // println!("Explode: left {idx_left}, right: {idx_right}");
    //         let mut idx_to_remove = Vec::new();
    //         // Left side
    //         if idx_left == 0 {
    //             nums[idx_left].depth -= 1;
    //             nums[idx_left].value = 0;
    //             nums[idx_left].offset -= 1;
    //         } else {
    //             let current_val = nums[idx_left].value;
    //             let current_offset = nums[idx_left].offset;
    //             let prev_left = &mut nums[idx_left - 1];
    //             if current_offset == 1 {
    //                 prev_left.value += current_val;
    //                 idx_to_remove.push(idx_left);
    //             } else {
    //                 prev_left.value += current_val;
    //                 nums[idx_left].depth -= 1;
    //                 nums[idx_left].value = 0;
    //                 nums[idx_left].offset -= 1;
    //             }
    //         }
    //
    //         // Right side
    //         if idx_right == nums.len() - 1 {
    //             nums[idx_right].depth -= 1;
    //             nums[idx_right].value = 0;
    //         } else {
    //             let current_value = nums[idx_right].value;
    //             let next_right = &mut nums[idx_right + 1];
    //             if next_right.offset == 1 {
    //                 next_right.value += current_value;
    //                 next_right.offset = 0;
    //                 idx_to_remove.push(idx_right);
    //             } else {
    //                 next_right.value += current_value;
    //                 next_right.offset -= 1;
    //                 nums[idx_right].depth -= 1;
    //                 nums[idx_right].value = 0;
    //             }
    //         }
    //
    //         // Remove items
    //         // Start at the right values
    //         for &idx in idx_to_remove.iter().rev() {
    //             // println!("Remove {idx}");
    //             nums.remove(idx);
    //         }
    //
    //         dbg!(nums);
    //         println!("==========================================================================");
    //     }
    //
    //     assert!(false);
    // }

    #[test]
    fn test_part_1() {
        // assert_eq!(calc_magnitude(BASIC_INPUT), 0);
        assert_eq!(calc_magnitude(INPUT_1), 129);
        // assert_eq!(calc_magnitude(INPUT_2), 4140);
    }
}
