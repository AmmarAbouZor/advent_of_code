def read_input_lines():
    with open("input.txt") as f:
        return f.read().splitlines()


def part_1(input: list[str]):
    valid_password = 0

    for line in input:
        chunks = line.split(" ")
        min_max = chunks[0].split("-")
        min = int(min_max[0])
        max = int(min_max[1])

        char = chunks[1][0]
        char_count = chunks[2].count(char)
        if char_count >= min and char_count <= max:
            valid_password += 1

    return valid_password


def part_2(input: list[str]):
    valid_password = 0

    for line in input:
        chunks = line.split(" ")
        indecies = chunks[0].split("-")
        idx_1 = int(indecies[0]) - 1
        idx_2 = int(indecies[1]) - 1

        char = chunks[1][0]
        found_1 = chunks[2][idx_1] == char
        found_2 = chunks[2][idx_2] == char

        if found_1 != found_2:
            valid_password += 1

    return valid_password


def run():
    input = read_input_lines()
    answer_1 = part_1(input)
    print(f"Part 1 answer {answer_1}")
    answer_2 = part_2(input)
    print(f"Part 2 answer {answer_2}")


run()
