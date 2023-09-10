from typing import Optional, Tuple
import copy


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

    def can_adjust(self) -> bool:
        return self.oper == "jmp" or (self.oper == "nop" and self.num != 0)

    def adjust(self):
        if self.oper == "nop":
            self.oper = "jmp"
        elif self.oper == "jmp":
            self.oper = "nop"
        else:
            raise AssertionError("operation must be nop or jmp")


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


def adjust_insts(instructions: list[Instruction], adjustments_set: set[int]):
    for idx, inst in enumerate(instructions):
        if idx in adjustments_set:
            continue
        if inst.can_adjust():
            inst.adjust()
            adjustments_set.add(idx)
            return
    raise AssertionError("Instructions must be solvable")


def try_solve_insts(instructions: list[Instruction]) -> Optional[int]:
    index, acc = 0, 0
    visited_idx = set()
    while True:
        visited_idx.add(index)
        idx_change, acc_change = instructions[index].apply()
        index += idx_change
        acc += acc_change
        if index >= len(instructions):
            return acc
        if index in visited_idx:
            return None


def fix_loop(input: str) -> int:
    instructions = parse_instructions(input)
    adjustments_set = set()
    while True:
        insts_copy = copy.deepcopy(instructions)
        adjust_insts(insts_copy, adjustments_set)
        answer = try_solve_insts(insts_copy)
        if answer is not None:
            return answer


def part_1():
    input = get_input()
    answer = get_acc_on_infinite(input)
    print(f"Part 1 answer is {answer}")


def part_2():
    input = get_input()
    answer = fix_loop(input)
    print(f"Part 2 answer is {answer}")


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
    acc = fix_loop(TEST_INPUT)
    assert 8 == acc, f"Part 2: Left 8, right {acc}"
    print("Tests passed")


test()
part_1()
part_2()
