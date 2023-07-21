#![allow(warnings, unused)]

use std::collections::HashMap;

use crate::utls::read_lines_from_file;

#[derive(Debug, Clone)]
enum Operation {
    Assign(String),
    Complement(String),
    And(String, String),
    Or(String, String),
    LeftShift(String, String),
    RightShift(String, String),
}

impl From<&str> for Operation {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.trim().split(' ').collect();
        match parts.len() {
            1 => Operation::Assign(parts[0].into()),
            2 => Operation::Complement(parts[1].into()),
            3 if parts[1] == "AND" => Operation::And(parts[0].into(), parts[2].into()),
            3 if parts[1] == "OR" => Operation::Or(parts[0].into(), parts[2].into()),
            3 if parts[1] == "LSHIFT" => Operation::LeftShift(parts[0].into(), parts[2].into()),
            3 if parts[1] == "RSHIFT" => Operation::RightShift(parts[0].into(), parts[2].into()),
            _ => panic!("invalid input:{text}"),
        }
    }
}

impl Operation {
    fn try_calculate(&self, map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Operation::Assign(text) => try_resolve_input(text, map),
            Operation::Complement(text) => try_resolve_input(text, map).map(|value| !value),
            Operation::And(text_1, text_2) => {
                if let (Some(value_1), Some(value_2)) = (
                    try_resolve_input(text_1, map),
                    try_resolve_input(text_2, map),
                ) {
                    Some(value_1 & value_2)
                } else {
                    None
                }
            }
            Operation::Or(text_1, text_2) => {
                if let (Some(value_1), Some(value_2)) = (
                    try_resolve_input(text_1, map),
                    try_resolve_input(text_2, map),
                ) {
                    Some(value_1 | value_2)
                } else {
                    None
                }
            }
            Operation::LeftShift(text_1, text_2) => {
                if let (Some(value_1), Some(value_2)) = (
                    try_resolve_input(text_1, map),
                    try_resolve_input(text_2, map),
                ) {
                    Some(value_1 << value_2)
                } else {
                    None
                }
            }
            Operation::RightShift(text_1, text_2) => {
                if let (Some(value_1), Some(value_2)) = (
                    try_resolve_input(text_1, map),
                    try_resolve_input(text_2, map),
                ) {
                    Some(value_1 >> value_2)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct OperationInfos {
    operation: Operation,
    output_name: String,
    resolved: bool,
}

impl From<&str> for OperationInfos {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split("->").collect();
        OperationInfos {
            operation: parts[0].into(),
            output_name: parts[1].trim().into(),
            resolved: false,
        }
    }
}

fn try_resolve_input(text: &str, map: &HashMap<String, u16>) -> Option<u16> {
    text.parse::<u16>()
        .ok()
        .or_else(|| map.get(text).map(|&value| value))
}

fn get_operations() -> Vec<OperationInfos> {
    read_lines_from_file(r"src/year_15/day_7.txt")
        .into_iter()
        .map(|line| line.as_str().into())
        .collect()
}

fn find_gate_signal(
    gate: &str,
    mut gates_map: HashMap<String, u16>,
    mut operations: Vec<OperationInfos>,
) -> Option<u16> {
    let mut has_unresolved = true;

    while has_unresolved {
        has_unresolved = false;
        for op in operations.iter_mut().filter(|op| !op.resolved) {
            if let Some(signal) = op.operation.try_calculate(&gates_map) {
                if op.output_name == gate {
                    return Some(signal);
                } else {
                    op.resolved = true;
                    gates_map.insert(op.output_name.clone(), signal);
                }
            } else {
                has_unresolved = true;
            }
        }
    }

    None
}

pub fn run() {
    let mut gates_map = HashMap::new();
    let mut operations = get_operations();

    let mut manipulated_operations: Vec<OperationInfos> = operations
        .iter()
        .filter(|op| op.output_name != "b")
        .cloned()
        .collect();

    let signal_a = find_gate_signal("a", gates_map, operations).unwrap();
    println!("signal for the gate a is {signal_a}");

    let mut gates_map = HashMap::new();
    gates_map.insert("b".into(), signal_a);
    let signal_a = find_gate_signal("a", gates_map, manipulated_operations).unwrap();
    println!("signal for the gate a after manipulation is {signal_a}");
}
