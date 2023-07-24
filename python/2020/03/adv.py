from typing import Tuple


def get_input_lines() -> list[str]:
    with open("input.txt") as f:
        return f.read().splitlines()


TEST_INPUT = """..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"""


class Position:
    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col


def calc_encounters(input: list[str], slop: Tuple[int, int]):
    width = len(input[0])
    height = len(input)
    count = 0
    pos = Position(0, 0)

    while pos.row < height:
        if input[pos.row][pos.col % width] == "#":
            count += 1
        pos.row += slop[1]
        pos.col += slop[0]

    return count


def part_1(input: list[str]) -> int:
    return calc_encounters(input, (3, 1))


def part_2(input: list[str]) -> int:
    slops = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    answer = 1
    for slop in slops:
        answer *= calc_encounters(input, slop)

    return answer


def test():
    input = TEST_INPUT.splitlines()
    count_1 = part_1(input)
    print(f"part 1 test answer is {count_1}")
    count_2 = part_2(input)
    print(f"part 2 test answer is {count_2}")


def main():
    input = get_input_lines()
    answer_1 = part_1(input)
    print(f"part 1 answer is {answer_1}")
    answer_2 = part_2(input)
    print(f"part 2 answer is {answer_2}")


test()
main()
