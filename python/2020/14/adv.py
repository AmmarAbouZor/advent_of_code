from typing import Optional


class Operation:
    def __init__(self, address: int, value: int) -> None:
        self.address = address
        self.value = value

    def __repr__(self) -> str:
        return f"(address: {self.address}, value: {self.value})"

    @classmethod
    def try_from_string(cls, line: str) -> Optional["Operation"]:
        if not line.startswith("mem"):
            return None

        line = line.removeprefix("mem[").strip()
        parts = line.split("] = ")
        return Operation(int(parts[0]), int(parts[1]))


class Program:
    def __init__(self, mask: str) -> None:
        self.mask = mask
        self.operations: list[Operation] = []

    def __repr__(self) -> str:
        return f"mask: {self.mask}, operations: {self.operations}"

    def apply_value(self, memory_map: dict[int, int]):
        and_mask = int(self.mask.replace("X", "1"), 2)
        or_mask = int(self.mask.replace("X", "0"), 2)

        for operation in self.operations:
            and_val = operation.value & and_mask
            full_val = and_val | or_mask
            memory_map[operation.address] = full_val


def parse_input(input: str) -> list[Program]:
    programs: list[Program] = []
    for line in input.splitlines():
        operation = Operation.try_from_string(line)
        if operation:
            programs[-1].operations.append(operation)
        else:
            mask = line.removeprefix("mask = ").strip()
            programs.append(Program(mask))

    return programs


def calc_sum_mask_value(input: str) -> int:
    programs = parse_input(input)
    memory_map: dict[int, int] = {}
    for program in programs:
        program.apply_value(memory_map)

    return sum(memory_map.values())


def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


def part_1(input: str):
    answer_1 = calc_sum_mask_value(input)
    print(f"Part 1 answer is {answer_1}")


def part_2(input: str):
    pass


def run_test():
    input = get_test_input()
    answer_1 = calc_sum_mask_value(input)
    assert answer_1 == 165, f"Expected: '{165}', Found: '{answer_1}'"
    answer_2 = 0
    assert answer_2 == 0, f"Expected: '{0}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
