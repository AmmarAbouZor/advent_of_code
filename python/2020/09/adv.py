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


def find_first_invalid(depth: int, input: str) -> int:
    nums = [int(line) for line in input.splitlines()]
    start_idx = 0
    end_idx = depth
    while True:
        if not has_two_sum(nums[end_idx], nums[start_idx:end_idx]):
            return nums[end_idx]
        start_idx += 1
        end_idx += 1


def find_contiguous_num(target: int, input: str) -> int:
    nums = [int(line) for line in input.splitlines()]
    for idx, num in enumerate(nums):
        sum = num
        moving_idx = idx
        while sum <= target:
            moving_idx += 1
            sum += nums[moving_idx]
            if sum == target:
                return min(nums[idx : moving_idx + 1]) + max(nums[idx : moving_idx + 1])

    raise ValueError("Unreachable")


def part_1(input: str) -> int:
    answer = find_first_invalid(25, input)
    print(f"Part 1 answer is {answer}")

    return answer


def part_2(input: str, target: int):
    answer = find_contiguous_num(target, input)
    print(f"Part 2 answer is {answer}")


def run_test():
    input = get_test_input()
    result_1 = find_first_invalid(5, input)
    assert result_1 == 127, f"expected {127}, result {result_1}"

    result_2 = find_contiguous_num(result_1, input)
    assert result_2 == 62, f"expected {62}, result {result_2}"

    print("test pass")


if __name__ == "__main__":
    run_test()
    input = get_input()
    target = part_1(input)
    part_2(input, target)
