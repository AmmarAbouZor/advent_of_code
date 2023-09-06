use crate::utls::read_text_from_file;

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

fn decode_version_sum(input: &str) -> usize {
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
        .map(|num| format!("{:04b}", num))
        .collect()
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

fn binary_to_num(slice: &[u8]) -> usize {
    let binary_str = std::str::from_utf8(slice).unwrap();

    usize::from_str_radix(binary_str, 2).unwrap()
}

fn part_1() {
    let input = read_text_from_file("21", "16");
    let answer = decode_version_sum(&input);

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

    #[test]
    fn test_part_1() {
        assert_eq!(decode_version_sum("8A004A801A8002F478"), 16);
        assert_eq!(decode_version_sum("620080001611562C8802118E34"), 12);
        assert_eq!(decode_version_sum("C0015000016115A2E0802F182340"), 23);
        assert_eq!(decode_version_sum("A0016C880162017C3686B18A3D4780"), 31);
    }
}
