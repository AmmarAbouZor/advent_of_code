use std::{error::Error, str::FromStr};

use crate::utls::read_text_from_file;

#[derive(Debug)]
struct IP {
    valid_secs: Vec<Vec<char>>,
    hypernet_secs: Vec<Vec<char>>,
}

impl FromStr for IP {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split('[');

        let mut valid_secs = vec![];
        let mut hypernet_secs = vec![];

        valid_secs.push(parts.next().unwrap().chars().collect());

        for part in parts {
            let (hyp, valid) = part.split_once(']').unwrap();
            hypernet_secs.push(hyp.chars().collect());
            valid_secs.push(valid.chars().collect());
        }

        Ok(IP {
            valid_secs,
            hypernet_secs,
        })
    }
}

struct Aba(char, char);
impl IP {
    fn support_tls(&self) -> bool {
        self.valid_secs.iter().any(|sec| IP::has_abba(sec))
            && self.hypernet_secs.iter().all(|sec| !IP::has_abba(sec))
    }

    fn has_abba(chars: &Vec<char>) -> bool {
        let mut index = 0;

        while index < chars.len() - 3 {
            if chars[index] == chars[index + 3]
                && chars[index + 1] == chars[index + 2]
                && chars[index] != chars[index + 1]
            {
                return true;
            }

            index += 1;
        }

        false
    }

    fn get_all_abas(&self) -> Vec<Aba> {
        let mut abas = vec![];

        for sec in self.valid_secs.iter() {
            for index in 0..sec.len() - 2 {
                if sec[index] == sec[index + 2] && sec[index] != sec[index + 1] {
                    abas.push(Aba(sec[index], sec[index + 1]));
                }
            }
        }

        abas
    }

    fn support_ssl(&self) -> bool {
        let abas = self.get_all_abas();

        abas.iter().any(|aba| {
            for sec in self.hypernet_secs.iter() {
                for i in 0..sec.len() - 2 {
                    if sec[i] == aba.1 && sec[i + 1] == aba.0 && sec[i + 2] == aba.1 {
                        return true;
                    }
                }
            }

            false
        })
    }
}

fn get_ips() -> Vec<IP> {
    read_text_from_file("16", "07")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_1() {
    let count = get_ips().iter().filter(|ip| ip.support_tls()).count();

    println!("IPs TLS count is {count}");
}

fn part_2() {
    let count = get_ips().iter().filter(|ip| ip.support_ssl()).count();

    println!("IPs SSL count is {count}");
}

pub fn run() {
    part_1();
    part_2();
}
