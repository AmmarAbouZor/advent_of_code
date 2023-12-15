use crate::utls::read_text_from_file;

fn calc_hash(text: &str) -> usize {
    text.as_bytes()
        .iter()
        .filter(|&&b| b != b'\n')
        .fold(0_usize, |mut acc, &b| {
            acc += b as usize;
            acc *= 17;
            acc %= 256;
            acc
        })
}

fn calc_sum(input: &str) -> usize {
    input.split(',').map(calc_hash).sum()
}

fn calc_focal_sum(input: &str) -> usize {
    let mut map: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for chunk in input.split(',') {
        let chunk = chunk.trim_end_matches('\n');
        // Remove case
        if let Some(rm_lbl) = chunk.strip_suffix('-') {
            let hash = calc_hash(rm_lbl);
            let que = map.get_mut(hash).unwrap();
            que.retain(|&(lbl, _)| lbl != rm_lbl)
        }
        // Add case
        else {
            let chunk = chunk.trim_matches('\n');
            let (lbl, num) = chunk.split_once('=').unwrap();
            let num = num.parse().unwrap();
            let hash = calc_hash(lbl);
            let que = map.get_mut(hash).unwrap();
            if let Some(entry) = que.iter_mut().find(|(l, _)| *l == lbl) {
                entry.1 = num;
            } else {
                que.push((lbl, num))
            }
        }
    }

    map.into_iter()
        .enumerate()
        .map(|(box_idx, lbls)| {
            lbls.iter()
                .enumerate()
                .map(|(lbl_idx, (_lbl, focal))| (box_idx + 1) * (lbl_idx + 1) * *focal)
                .sum::<usize>()
        })
        .sum()
}

fn part_1(input: &str) {
    let answer = calc_sum(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {
    let answer = calc_focal_sum(input);

    println!("Part 2 asnwer is {answer}");
}

pub fn run() {
    let input = read_text_from_file("23", "15");
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_solution() {
        assert_eq!(calc_sum(INPUT), 1320);
        assert_eq!(calc_focal_sum(INPUT), 145);
    }
}

