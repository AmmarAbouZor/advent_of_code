def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


def has_two_sum(target: int, nums: list[int]) -> bool:
    diff_set = set()
    for num in nums:
        if num in diff_set:
            return True
        diff_set.add(target - num)

    return False


def find_first_invalid(depth: int, input: str):
    nums = [int(line) for line in input.splitlines()]
    start_idx = 0
    end_idx = depth
    while True:
        if not has_two_sum(nums[end_idx], nums[start_idx:end_idx]):
            return nums[end_idx]
        start_idx += 1
        end_idx += 1


def part_1():
    answer = find_first_invalid(25, get_input())
    print(f"Part 1 answer is {answer}")


def part_2():
    pass


def run_test():
    result_1 = find_first_invalid(5, get_test_input())
    assert result_1 == 127, f"expected {127}, result {result_1}"

    print("test pass")


if __name__ == "__main__":
    run_test()
    part_1()
    part_2()
