from re import error


def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


class Instruction:
    def __init__(self, command: str, num: int) -> None:
        self.command = command
        self.num = num

    @classmethod
    def from_string(cls, line: str) -> "Instruction":
        command = line[0]
        num = int(line[1:])

        return Instruction(command, num)

    def __repr__(self) -> str:
        return f"command: {self.command}, num: {self.num}"


class StateDirection:
    def __init__(self) -> None:
        self.pos_x = 0
        self.pos_y = 0
        self.direction = "E"

    def __repr__(self) -> str:
        return f"Position ({self.pos_x}, {self.pos_y}), Direction: {self.direction}"

    def turn_left(self):
        if self.direction == "E":
            self.direction = "N"
        elif self.direction == "N":
            self.direction = "W"
        elif self.direction == "W":
            self.direction = "S"
        elif self.direction == "S":
            self.direction = "E"
        else:
            raise Exception(f"Invalid direction: {self.direction}")

    def turn_right(self):
        if self.direction == "E":
            self.direction = "S"
        elif self.direction == "S":
            self.direction = "W"
        elif self.direction == "W":
            self.direction = "N"
        elif self.direction == "N":
            self.direction = "E"
        else:
            raise Exception(f"Invalid direction: {self.direction}")

    def apply_instruction(self, inst: Instruction):
        if inst.command == "N":
            self.pos_y += inst.num
        elif inst.command == "S":
            self.pos_y -= inst.num
        elif inst.command == "E":
            self.pos_x += inst.num
        elif inst.command == "W":
            self.pos_x -= inst.num
        elif inst.command == "L":
            count = int(inst.num / 90)
            for _ in range(count):
                self.turn_left()
        elif inst.command == "R":
            count = int(inst.num / 90)
            for _ in range(count):
                self.turn_right()
        elif inst.command == "F":
            new_inst = Instruction(self.direction, inst.num)
            self.apply_instruction(new_inst)
        else:
            raise Exception(f"Invalid instruction command: {inst.command}")


def get_distance_direction(input: str) -> int:
    insts = [Instruction.from_string(line) for line in input.splitlines()]
    state = StateDirection()
    for inst in insts:
        state.apply_instruction(inst)

    return abs(state.pos_x) + abs(state.pos_y)


def part_1(input: str):
    answer_1 = get_distance_direction(input)

    print(f"Part 1 answer is {answer_1}")


def part_2(input: str):
    pass


def run_test():
    input = get_test_input()
    answer_1 = get_distance_direction(input)
    assert answer_1 == 25, f"Expected: '{25}', Found: '{answer_1}'"
    answer_2 = 0
    assert answer_2 == 0, f"Expected: '{0}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
