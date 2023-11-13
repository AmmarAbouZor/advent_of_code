def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


def get_min_diff(id: int, target: int) -> int:
    num_before = target // id
    first_arrive = id * (num_before + 1)

    return first_arrive - target


def get_earliest(input: str) -> int:
    lines = input.splitlines()
    target = int(lines[0])
    min_diff = min(
        (get_min_diff(int(num), target), int(num))
        for num in lines[1].split(",")
        if num != "x"
    )

    return min_diff[0] * min_diff[1]


def part_1(input: str):
    answer_1 = get_earliest(input)

    print(f"Part 1 answer is {answer_1}")


def part_2(input: str):
    pass


def run_test():
    input = get_test_input()
    answer_1 = get_earliest(input)
    assert answer_1 == 295, f"Expected: '{295}', Found: '{answer_1}'"
    answer_2 = 0
    assert answer_2 == 0, f"Expected: '{0}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
