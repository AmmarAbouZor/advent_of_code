def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


def calc_diff_product(input: str) -> int:
    nums = [int(num) for num in input.splitlines()]
    nums.sort()

    nums.append(nums[-1] + 3)

    count_1 = 0
    count_3 = 0
    current = 0

    for num in nums:
        diff = num - current
        if diff == 1:
            count_1 += 1
        elif diff == 3:
            count_3 += 1
        current = num

    return count_1 * count_3


def part_1(input: str):
    answer = calc_diff_product(input)

    print(f"Part 1 answer is {answer}")


def part_2(input: str):
    pass


def run_test():
    input = get_test_input()
    answer_1 = calc_diff_product(input)
    expected_1 = 22 * 10
    assert answer_1 == expected_1, f"Expected: '{expected_1}', Found: '{answer_1}'"
    answer_2 = 0
    assert answer_2 == 0, f"Expected: '{0}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
