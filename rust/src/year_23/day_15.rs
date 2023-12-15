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

fn part_1(input: &str) {
    let answer = calc_sum(input);

    println!("Part 1 answer is {answer}");
}

fn part_2(input: &str) {}

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
    }
}

