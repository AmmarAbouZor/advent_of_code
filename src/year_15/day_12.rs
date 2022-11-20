#![allow(warnings, unused)]

use serde_json::{Map, Result, Value};
use std::{fs, io};

fn read_input_to_string() -> String {
    fs::read_to_string(r"src/year_15/day_12.txt").unwrap()
}

fn part_1() {
    let chars: Vec<char> = read_input_to_string().chars().collect();
    let mut sum = 0i32;
    let mut index = 0;
    while index < chars.len() {
        if chars[index].is_numeric() || chars[index] == '-' {
            let start_index = index;
            index += 1;
            while chars[index].is_numeric() {
                index += 1;
            }
            let num_text = chars[start_index..index].iter().collect::<String>();
            let num: i32 = num_text.parse().unwrap();
            sum += num;
        } else {
            index += 1;
        }
    }

    println!("Sum is {sum}");
}

fn get_root_value() -> Value {
    let file = fs::File::open(r"src/year_15/day_12.txt").unwrap();

    let reader = io::BufReader::new(file);
    serde_json::from_reader::<_, Value>(reader).unwrap()
}

fn calc_sum<F>(val: &Value, sum: &mut i64, is_object_valid: F)
where
    F: Fn(&Map<String, Value>) -> bool + Copy,
{
    match val {
        Value::Number(num) => *sum += num.as_i64().unwrap(),
        Value::Array(arr) => arr
            .iter()
            .for_each(|value| calc_sum(value, sum, is_object_valid)),
        Value::Object(obj) => {
            if is_object_valid(obj) {
                obj.values()
                    .for_each(|value| calc_sum(value, sum, is_object_valid));
            }
        }
        _ => {}
    }
}

fn part_1_serde() {
    let root_val = get_root_value();
    let mut sum = 0;
    calc_sum(&root_val, &mut sum, |_| true);

    println!("sum with serde is {sum}");
}

fn part_2_serde() {
    let root_val = get_root_value();
    let mut sum = 0;
    calc_sum(&root_val, &mut sum, |map| {
        map.values().all(|value| {
            if let Value::String(text) = value {
                text != "red"
            } else {
                true
            }
        })
    });

    println!("sum with serde with no reds is {sum}");
}

pub fn run() {
    // part_1();
    part_1_serde();
    part_2_serde();
}
