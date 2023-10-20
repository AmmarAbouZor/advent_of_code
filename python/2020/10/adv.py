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


def calc_distinct_ways(input: str) -> int:
    nums = [int(num) for num in input.splitlines()]
    nums.sort()

    ways_map = {}
    ways_map[0] = 1

    for num in nums:
        dist_ways = (
            ways_map.get(num - 1, 0)
            + ways_map.get(num - 2, 0)
            + ways_map.get(num - 3, 0)
        )
        ways_map[num] = dist_ways

    return ways_map[nums[-1]]


def part_2(input: str):
    answer = calc_distinct_ways(input)

    print(f"Part 2 answer is {answer}")


def run_test():
    input = get_test_input()
    answer_1 = calc_diff_product(input)
    expected_1 = 22 * 10
    assert answer_1 == expected_1, f"Expected: '{expected_1}', Found: '{answer_1}'"
    answer_2 = calc_distinct_ways(input)
    expected_2 = 19208
    assert answer_2 == expected_2, f"Expected: '{expected_2}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
