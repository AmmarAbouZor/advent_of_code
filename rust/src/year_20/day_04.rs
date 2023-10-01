use crate::utls::read_text_from_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldType {
    BirthYear,
    IssueYear,
    ExpirationDate,
    Height,
    HairColor,
    EyeColor,
    PassID,
    CountryID,
}

#[derive(Debug, Clone)]
struct Field<'a> {
    typ: FieldType,
    value: &'a str,
}

impl<'a> From<&'a str> for Field<'a> {
    fn from(input: &'a str) -> Self {
        use FieldType as F;
        let (key, value) = input.split_once(':').unwrap();
        let typ = match key {
            "byr" => F::BirthYear,
            "iyr" => F::IssueYear,
            "eyr" => F::ExpirationDate,
            "hgt" => F::Height,
            "hcl" => F::HairColor,
            "ecl" => F::EyeColor,
            "pid" => F::PassID,
            "cid" => F::CountryID,
            invalid => unreachable!("Invalid input: '{invalid}'"),
        };

        Field { typ, value }
    }
}

#[derive(Debug)]
struct Passport<'a> {
    fields: Vec<Field<'a>>,
}

impl<'a> Passport<'a> {
    fn has_required_fields(&self) -> bool {
        let mut fields = [false; 7];
        for Field { typ, .. } in self.fields.iter() {
            if matches!(typ, FieldType::CountryID) {
                continue;
            }

            fields[*typ as usize] = true;
        }

        fields.iter().all(|&val| val)
    }
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(input: &'a str) -> Self {
        let fields = input
            .lines()
            .flat_map(|line| line.split_whitespace())
            .map(Field::from)
            .collect();
        Passport { fields }
    }
}

fn get_valid_passes(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::from)
        .filter(Passport::has_required_fields)
        .count()
}

fn part_1() {
    let input = read_text_from_file("20", "04");
    let answer = get_valid_passes(&input);

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

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_solution() {
        assert_eq!(get_valid_passes(INPUT), 2);
    }
}

