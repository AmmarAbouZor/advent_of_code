#![allow(warnings, unused)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_min_num() {
        assert_eq!(get_min_num("abcdef", "00000"), 609043);
        assert_eq!(get_min_num("pqrstuv", "00000"), 1048970);
    }
}

fn get_hash(text: &str, number: usize) -> String {
    format!("{:x}", md5::compute(format!("{text}{number}").as_bytes()))
}

fn get_min_num(text: &str, start: &str) -> usize {
    let mut counter = 0usize;
    loop {
        let hash = get_hash(text, counter);
        if hash.starts_with(start) {
            return counter;
        }
        counter += 1;
    }
}

pub fn run() {
    let hash_5 = get_min_num("yzbqklnj", "00000");
    println!("hash_5 is: {hash_5}");
    let hash_6 = get_min_num("yzbqklnj", "000000");
    println!("hash_6 is: {hash_6}");
}
