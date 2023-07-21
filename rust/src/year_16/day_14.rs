fn calc_hash_simple(text: &str, number: usize) -> String {
    format!("{:x}", md5::compute(format!("{text}{number}").as_bytes()))
}

fn try_get_same_char(input: &str, count: usize) -> Option<char> {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() - count + 1 {
        let mut same_char = None;
        for j in 0..count - 1 {
            if chars[i + j] != chars[i + j + 1] {
                same_char = None;
                break;
            }
            same_char = Some(chars[i]);
        }

        if same_char.is_some() {
            return same_char;
        }
    }

    None
}

fn has_same_char(input: &str, count: usize, ch: char) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() - count + 1 {
        let mut same = false;
        for j in 0..count {
            if chars[i + j] != ch {
                same = false;
                break;
            }
            same = true;
        }

        if same {
            return true;
        }
    }

    false
}

fn get_hash<F>(seed: &str, index: usize, hashes: &mut Vec<String>, calc_hash: F) -> String
where
    F: Fn(&str, usize) -> String,
{
    if let Some(hash) = hashes.get(index) {
        hash.to_owned()
    } else {
        let hash = calc_hash(seed, index);

        hashes.push(hash.clone());

        assert_eq!(hashes.len(), index + 1);

        hash
    }
}

fn calc_is_key<F>(seed: &str, index: usize, hashes: &mut Vec<String>, calc_hash: F) -> bool
where
    F: Fn(&str, usize) -> String,
{
    let hash = get_hash(seed, index, hashes, &calc_hash);

    if let Some(ch) = try_get_same_char(&hash, 3) {
        for i in 1..1001 {
            let ha = get_hash(seed, index + i, hashes, &calc_hash);
            if has_same_char(&ha, 5, ch) {
                return true;
            }
        }
    }

    false
}

fn calc_index_for_target_count<F>(seed: &str, target: usize, calc_hash: F) -> usize
where
    F: Fn(&str, usize) -> String,
{
    let mut hashes = Vec::new();

    let mut valid_count = 0;

    let mut index = 0;
    loop {
        if calc_is_key(seed, index, &mut hashes, &calc_hash) {
            valid_count += 1;
            if valid_count == target {
                return index;
            }
        }

        index += 1;
    }
}

fn part_1() {
    let index = calc_index_for_target_count("ngcjuoqr", 64, calc_hash_simple);

    println!("part 1: index of 64th key is {index}");
}

fn calc_hash_recr(text: &str, number: usize) -> String {
    let mut hash = calc_hash_simple(text, number);

    for _ in 0..2016 {
        hash = format!("{:x}", md5::compute(hash.as_bytes()));
    }

    hash
}

fn part_2() {
    let index = calc_index_for_target_count("ngcjuoqr", 64, calc_hash_recr);

    println!("part_2: index of 64th key is {index}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_same() {
        let test = calc_hash_simple("abc", 18);
        assert_eq!(try_get_same_char(&test, 3), Some('8'));
        assert_eq!(try_get_same_char(&test, 4), None);

        let test_5 = calc_hash_simple("abc", 816);
        assert!(has_same_char(&test_5, 5, 'e'));
        assert!(!has_same_char(&test_5, 6, 'e'));
    }

    #[test]
    #[ignore]
    fn test_calc_index_simple() {
        assert_eq!(
            calc_index_for_target_count("abc", 64, calc_hash_simple),
            22728
        );
    }
}
