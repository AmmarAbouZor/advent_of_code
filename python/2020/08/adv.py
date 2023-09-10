from typing import Tuple


def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


class Instruction:
    def __init__(self, oper: str, num: int) -> None:
        self.oper = oper
        self.num = num

    def __repr__(self) -> str:
        return f"(oper: {self.oper}, num: {self.num})"

    def apply(self) -> Tuple[int, int]:
        if self.oper == "nop":
            return 1, 0
        if self.oper == "acc":
            return 1, self.num
        if self.oper == "jmp":
            return self.num, 0

        raise ValueError("Invalid operation name")


def parse_instructions(input: str) -> list[Instruction]:
    instructions = []
    for line in input.splitlines():
        parts = line.split(" ")
        inst = Instruction(parts[0], int(parts[1]))
        instructions.append(inst)

    return instructions


def get_acc_on_infinite(input: str) -> int:
    instructions = parse_instructions(input)
    index, acc = 0, 0
    visited_idx = set()
    while index not in visited_idx:
        visited_idx.add(index)
        idx_change, acc_change = instructions[index].apply()
        index += idx_change
        acc += acc_change

    return acc


def part_1():
    input = get_input()
    answer = get_acc_on_infinite(input)
    print(f"Part 1 answer is {answer}")


TEST_INPUT = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""


def test():
    acc = get_acc_on_infinite(TEST_INPUT)
    assert 5 == acc, f"Part 1: Left 5, right {acc}"
    print("Tests passed")


test()
part_1()
