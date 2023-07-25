import re


def get_input_chunks() -> list[str]:
    with open("input.txt") as f:
        return f.read().split("\n\n")


REQUIRED_KEYS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
VALID_COLORS = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


def parse_pass(txt: str) -> dict[str, str]:
    splits = (chunk.split(":") for chunk in txt.split())
    return {split[0]: split[1] for split in splits}


def part_1(input: list[str]) -> int:
    count = 0

    for batch in input:
        pass_data = parse_pass(batch)
        if all(key in pass_data.keys() for key in REQUIRED_KEYS):
            count += 1

    return count


def validat_pass(txt: str) -> bool:
    pass_data = parse_pass(txt)

    # byr
    byr = pass_data.get("byr")
    if byr == None or len(byr) != 4 or not byr.isdecimal():
        return False
    byr_year = int(byr)
    if byr_year < 1920 or byr_year > 2002:
        return False

    # iyr
    iyr = pass_data.get("iyr")
    if iyr == None or len(iyr) != 4 or not iyr.isdecimal():
        return False
    iyr_num = int(iyr)
    if iyr_num < 2010 or iyr_num > 2020:
        return False

    # eyr
    eyr = pass_data.get("eyr")
    if eyr == None or len(eyr) != 4 or not eyr.isdecimal():
        return False
    eyr_num = int(eyr)
    if eyr_num < 2020 or eyr_num > 2030:
        return False

    # hgt
    hgt = pass_data.get("hgt")
    if hgt == None or len(hgt) < 4:
        return False
    if not hgt[:-2].isdecimal():
        return False
    height = int(hgt[:-2])

    if hgt[-2:] == "cm":
        if height < 150 or height > 193:
            return False
    elif hgt[-2:] == "in":
        if height < 59 or height > 76:
            return False
    else:
        return False

    # hcl
    hcl = pass_data.get("hcl")
    if hcl == None:
        return False
    if not re.match(r"^#[0-9a-f]{6}", hcl):
        return False

    # ecl
    ecl = pass_data.get("ecl")
    if ecl == None or ecl not in VALID_COLORS:
        return False

    # pid
    pid = pass_data.get("pid")
    if pid == None or len(pid) != 9 or not pid.isdecimal():
        return False

    return True


def part_2(input: list[str]) -> int:
    return len(list(filter(validat_pass, input)))


def main():
    input = get_input_chunks()
    answer_1 = part_1(input)
    print(f"Part 1 answer is {answer_1}")
    answer_2 = part_2(input)
    print(f"Part 2 answer is {answer_2}")


main()
