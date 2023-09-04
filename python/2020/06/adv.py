def get_input_lines() -> str:
    with open("input.txt") as f:
        return f.read()


def calc_all_answers_sum(input: str) -> int:
    groups = input.split("\n\n")

    sum = 0
    for group in groups:
        chars_set = {ch for ch in group if ch != "\n"}
        sum += len(chars_set)

    return sum


def calc_intersection_sum(input: str) -> int:
    groups = input.split("\n\n")

    return sum((calc_intersect_group(group) for group in groups))


def calc_intersect_group(group: str) -> int:
    lines = group.splitlines()
    sets = [set(line) for line in lines]

    return len(set.intersection(*sets))


def part_1():
    input = get_input_lines()
    answer = calc_all_answers_sum(input)

    print(f"Part 1 answer is {answer}")


def part_2():
    input = get_input_lines()
    answer = calc_intersection_sum(input)

    print(f"Part 2 answer is {answer}")


TEST_INPUT = """abc

a
b
c

ab
ac

a
a
a
a

b
"""


def test():
    sums = calc_all_answers_sum(TEST_INPUT)
    assert 11 == sums, f"Left: {11}, Right: {sums}"
    inter_sum = calc_intersection_sum(TEST_INPUT)
    assert 6 == inter_sum, f"Left: {6}, Right: {inter_sum}"
    print("Tests passed")


test()
part_1()
part_2()
