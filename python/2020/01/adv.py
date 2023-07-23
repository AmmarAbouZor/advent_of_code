TARGET = 2020


def get_input_lines():
    with open("input.txt") as f:
        return f.read().splitlines()


def part_1(input_lines: list[str]):
    nums = (int(line) for line in input_lines)
    map = {}

    # The Idea to save the target of the each number as a key in a map which with the number itself
    # This reduces the time complexity to O(n)
    for num in nums:
        num_2 = map.get(num)

        if num_2 == None:
            map[TARGET - num] = num
        else:
            return num * num_2


# Just normal iteration with a Time complexity of huge O(n3)
def part_2(input_lines: list[str]):
    nums = [int(line) for line in input_lines]
    for num_1 in nums:
        for num_2 in nums:
            for num_3 in nums:
                if num_1 == num_2 or num_1 == num_3 or num_2 == num_3:
                    continue
                if num_1 + num_2 + num_3 == TARGET:
                    return num_1 * num_2 * num_3


def run():
    lines = get_input_lines()
    answer_1 = part_1(lines)
    print(f"Part 1 answer is {answer_1}")
    answer_2 = part_2(lines)
    print(f"Part 2 answer is {answer_2}")


run()
