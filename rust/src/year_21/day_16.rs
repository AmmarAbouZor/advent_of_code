use crate::utls::read_text_from_file;
use std::fmt::Write;

#[derive(Debug)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl From<usize> for PacketType {
    fn from(value: usize) -> Self {
        match value {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            4 => PacketType::Literal,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::Equal,
            _ => unreachable!("Invalid input for PacketType"),
        }
    }
}

fn calc_decoded_versions_sum(input: &str) -> usize {
    let input = input.trim();
    let binary = hex_to_binary(input);
    let bin_slice = binary.as_bytes();

    // versions as vector for debugging. It can be a normal usize count too
    let mut versions = Vec::new();

    let mut idx = 0;

    decode_packet_version(bin_slice, &mut idx, &mut versions);

    versions.into_iter().sum()
}

#[inline]
fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        .map(|ch| ch.to_digit(16).unwrap())
        .fold(String::new(), |mut output, num| {
            let _ = write!(output, "{:04b}", num);
            output
        })
}

fn decode_packet_version(binary: &[u8], idx: &mut usize, versions: &mut Vec<usize>) {
    let version = binary_to_num(&binary[*idx..*idx + 3]);
    *idx += 3;
    versions.push(version);

    let packet_type: PacketType = binary_to_num(&binary[*idx..*idx + 3]).into();
    *idx += 3;

    match packet_type {
        PacketType::Literal => {
            let _ = parse_literal(binary, idx);
        }
        _ => parse_operator_for_version(binary, idx, versions),
    }
}

/// This returns the sum for the literal and set the index on the next position
fn parse_literal(binary: &[u8], idx: &mut usize) -> usize {
    let mut literal_value = String::new();

    let mut reached_end = false;

    while !reached_end {
        reached_end = binary[*idx] == b'0';
        let binary_chunk = std::str::from_utf8(&binary[*idx + 1..*idx + 5]).unwrap();
        literal_value.push_str(binary_chunk);
        *idx += 5;
    }

    usize::from_str_radix(literal_value.as_str(), 2).unwrap()
}

fn parse_operator_for_version(binary: &[u8], idx: &mut usize, versions: &mut Vec<usize>) {
    let lenght_type_id = binary[*idx];
    *idx += 1;
    if lenght_type_id == b'0' {
        let bit_lenght = binary_to_num(&binary[*idx..*idx + 15]);
        *idx += 15;
        let start_idx = *idx;
        while *idx < start_idx + bit_lenght {
            decode_packet_version(binary, idx, versions);
        }
    } else {
        let packets_count = binary_to_num(&binary[*idx..*idx + 11]);
        *idx += 11;
        for _ in 0..packets_count {
            decode_packet_version(binary, idx, versions);
        }
    }
}

fn calc_decoded_result(input: &str) -> usize {
    let input = input.trim();
    let binary = hex_to_binary(input);
    let bin_slice = binary.as_bytes();

    let mut idx = 0;

    decode_packet_result(bin_slice, &mut idx)
}

fn decode_packet_result(binary: &[u8], idx: &mut usize) -> usize {
    // Version is irrelevant here
    *idx += 3;

    let packet_type: PacketType = binary_to_num(&binary[*idx..*idx + 3]).into();
    *idx += 3;

    match packet_type {
        PacketType::Literal => parse_literal(binary, idx),
        PacketType::Sum => {
            let resutls = parse_operator_for_result(binary, idx);
            resutls.into_iter().sum()
        }
        PacketType::Product => {
            let resutls = parse_operator_for_result(binary, idx);
            resutls.into_iter().product()
        }
        PacketType::Minimum => {
            let resutls = parse_operator_for_result(binary, idx);
            resutls.into_iter().min().unwrap()
        }
        PacketType::Maximum => {
            let resutls = parse_operator_for_result(binary, idx);
            resutls.into_iter().max().unwrap()
        }
        PacketType::GreaterThan => {
            let resutls = parse_operator_for_result(binary, idx);
            assert_eq!(resutls.len(), 2);
            if resutls[0] > resutls[1] {
                1
            } else {
                0
            }
        }
        PacketType::LessThan => {
            let resutls = parse_operator_for_result(binary, idx);
            assert_eq!(resutls.len(), 2);
            if resutls[0] < resutls[1] {
                1
            } else {
                0
            }
        }
        PacketType::Equal => {
            let resutls = parse_operator_for_result(binary, idx);
            assert_eq!(resutls.len(), 2);
            if resutls[0] == resutls[1] {
                1
            } else {
                0
            }
        }
    }
}

fn parse_operator_for_result(binary: &[u8], idx: &mut usize) -> Vec<usize> {
    let mut results = Vec::new();
    let lenght_type_id = binary[*idx];
    *idx += 1;
    if lenght_type_id == b'0' {
        let bit_lenght = binary_to_num(&binary[*idx..*idx + 15]);
        *idx += 15;
        let start_idx = *idx;
        while *idx < start_idx + bit_lenght {
            let result = decode_packet_result(binary, idx);
            results.push(result);
        }
    } else {
        let packets_count = binary_to_num(&binary[*idx..*idx + 11]);
        *idx += 11;
        for _ in 0..packets_count {
            let result = decode_packet_result(binary, idx);
            results.push(result);
        }
    }

    results
}

fn binary_to_num(slice: &[u8]) -> usize {
    let binary_str = std::str::from_utf8(slice).unwrap();

    usize::from_str_radix(binary_str, 2).unwrap()
}

fn part_1() {
    let input = read_text_from_file("21", "16");
    let answer = calc_decoded_versions_sum(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("21", "16");
    let answer = calc_decoded_result(&input);

    println!("Part 2 answer is {answer}");
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
        assert_eq!(calc_decoded_versions_sum("8A004A801A8002F478"), 16);
        assert_eq!(calc_decoded_versions_sum("620080001611562C8802118E34"), 12);
        assert_eq!(
            calc_decoded_versions_sum("C0015000016115A2E0802F182340"),
            23
        );
        assert_eq!(
            calc_decoded_versions_sum("A0016C880162017C3686B18A3D4780"),
            31
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_decoded_result("C200B40A82"), 3);
        assert_eq!(calc_decoded_result("04005AC33890"), 54);
        assert_eq!(calc_decoded_result("880086C3E88112"), 7);
        assert_eq!(calc_decoded_result("CE00C43D881120"), 9);
        assert_eq!(calc_decoded_result("D8005AC2A8F0"), 1);
        assert_eq!(calc_decoded_result("F600BC2D8F"), 0);
        assert_eq!(calc_decoded_result("9C005AC2F8F0"), 0);
        assert_eq!(calc_decoded_result("9C0141080250320F1802104A08"), 1);
    }
}
