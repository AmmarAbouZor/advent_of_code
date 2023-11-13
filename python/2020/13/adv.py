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


def calc_earliest_matching(input: str) -> int:
    lines = input.splitlines()
    buses = [
        (idx, int(num)) for idx, num in enumerate(lines[1].split(",")) if num != "x"
    ]

    lcm = 1
    time = 0
    for i in range(len(buses) - 1):
        bus = buses[i + 1][1]
        idx = buses[i + 1][0]
        lcm *= buses[i][1]
        while (time + idx) % bus != 0:
            time += lcm

    return time


def part_2(input: str):
    answer_2 = calc_earliest_matching(input)

    print(f"Part 2 answer is {answer_2}")


def run_test():
    input = get_test_input()
    answer_1 = get_earliest(input)
    assert answer_1 == 295, f"Expected: '{295}', Found: '{answer_1}'"
    answer_2 = calc_earliest_matching(input)
    assert answer_2 == 1068781, f"Expected: '{1068781}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
