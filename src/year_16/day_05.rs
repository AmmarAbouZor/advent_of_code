fn get_hash(text: &str, number: usize) -> String {
    format!("{:x}", md5::compute(format!("{text}{number}").as_bytes()))
}

fn get_password(text: &str, start: &str) -> String {
    let mut counter = 0usize;
    let mut password = String::new();
    while password.len() < 8 {
        let hash = get_hash(text, counter);
        if hash.starts_with(start) {
            password.push_str(&hash[5..6])
        }
        counter += 1;
    }

    password
}

fn get_password_2(text: &str, start: &str) -> String {
    let mut counter = 0usize;
    let mut password = ['!'; 8];
    while password.contains(&'!') {
        let hash = get_hash(text, counter);
        if hash.starts_with(start) {
            if let Ok(index) = &hash[5..6].parse::<usize>() {
                if (0..8).contains(index) && password[*index] == '!' {
                    password[*index] = hash.chars().nth(6).unwrap();
                }
            }
        }
        counter += 1;
    }

    password.iter().collect()
}

fn part_1() {
    let password = get_password("ugkcyxxp", "00000");

    println!("password is {password}");
}

fn part_2() {
    let password = get_password_2("ugkcyxxp", "00000");

    println!("password part 2 is {password}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_password() {
        assert_eq!(get_password("abc", "00000"), String::from("18f47a30"));
    }

    #[test]
    #[ignore]
    fn test_get_password_2() {
        assert_eq!(get_password_2("abc", "00000"), String::from("05ace8e3"));
    }
}
