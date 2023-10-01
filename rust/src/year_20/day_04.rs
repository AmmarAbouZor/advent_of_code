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

impl<'a> Field<'a> {
    fn is_valid(&self) -> bool {
        match self.typ {
            FieldType::BirthYear => self
                .value
                .parse::<u32>()
                .is_ok_and(|num| (1920..=2002).contains(&num)),
            FieldType::IssueYear => self
                .value
                .parse::<u32>()
                .is_ok_and(|num| (2010..=2020).contains(&num)),
            FieldType::ExpirationDate => self
                .value
                .parse::<u32>()
                .is_ok_and(|num| (2020..=2030).contains(&num)),
            FieldType::Height => {
                let (num, unit) = self.value.split_at(self.value.len() - 2);
                num.parse::<u32>().is_ok_and(|num| match unit {
                    "cm" => (150..=193).contains(&num),
                    "in" => (59..=76).contains(&num),
                    _invalid => false,
                })
            }
            FieldType::HairColor => {
                self.value.len() == 7
                    && self
                        .value
                        .bytes()
                        .skip(1)
                        .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
            }
            FieldType::EyeColor => {
                matches!(
                    self.value,
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            }
            FieldType::PassID => self.value.len() == 9 && self.value.parse::<u32>().is_ok(),
            FieldType::CountryID => true,
        }
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

    fn validate(&self) -> bool {
        self.has_required_fields() && self.fields.iter().all(Field::is_valid)
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

fn get_passes_with_required_fields(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::from)
        .filter(Passport::has_required_fields)
        .count()
}

fn get_valid_passes(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::from)
        .filter(Passport::validate)
        .count()
}

fn part_1() {
    let input = read_text_from_file("20", "04");
    let answer = get_passes_with_required_fields(&input);

    println!("Part 1 answer is {answer}");
}

fn part_2() {
    let input = read_text_from_file("20", "04");
    let answer = get_valid_passes(&input);

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

        assert_eq!(get_passes_with_required_fields(INPUT), 2);
    }

    #[test]
    fn test_invalid_pass() {
        const INPUT: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(get_valid_passes(INPUT), 0);
    }

    #[test]
    fn test_valid_passes() {
        const INPUT: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(get_valid_passes(INPUT), 4);
    }
}

