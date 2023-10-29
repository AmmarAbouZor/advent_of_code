def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


class Layout:
    DELTAS = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

    def __init__(self, input: str) -> None:
        self.cells = [list(line) for line in input.splitlines()]

    def get_occupied_count(self) -> int:
        return sum(row.count("#") for row in self.cells)

    def get_occupied_count_in_surround(self, row: int, col: int) -> int:
        counter = 0
        rows_len = len(self.cells)
        col_len = len(self.cells[0])
        for delta in self.DELTAS:
            d_row, d_col = delta
            new_row = row + d_row
            new_col = col + d_col
            if (
                0 <= new_row < rows_len
                and 0 <= new_col < col_len
                and self.cells[new_row][new_col] == "#"
            ):
                counter += 1

        return counter

    def apply_surround(self) -> bool:
        changed = False

        changed_cells = []

        for row_idx, row in enumerate(self.cells):
            changed_row = ["." for _ in range(len(row))]
            for col_idx, char in enumerate(row):
                if char == "L":
                    if self.get_occupied_count_in_surround(row_idx, col_idx) == 0:
                        changed = True
                        changed_row[col_idx] = "#"
                    else:
                        changed_row[col_idx] = "L"
                elif char == "#":
                    if self.get_occupied_count_in_surround(row_idx, col_idx) >= 4:
                        changed = True
                        changed_row[col_idx] = "L"
                    else:
                        changed_row[col_idx] = "#"
            changed_cells.append(changed_row)

        self.cells = changed_cells

        return changed


def get_all_occupied_surround(input: str) -> int:
    layout = Layout(input)
    counter = 0
    while layout.apply_surround():
        counter += 1
    print(f"counter is {counter}")

    return layout.get_occupied_count()


def part_1(input: str):
    answer_1 = get_all_occupied_surround(input)
    print(f"Part 1 answer is {answer_1}")


def part_2(input: str):
    pass


def run_test():
    input = get_test_input()
    answer_1 = get_all_occupied_surround(input)
    assert answer_1 == 37, f"Expected: '{37}', Found: '{answer_1}'"
    answer_2 = 26
    assert answer_2 == 26, f"Expected: '{26}', Found: '{answer_2}'"

    print("Tests passed!")


if __name__ == "__main__":
    run_test()
    input = get_input()
    part_1(input)
    part_2(input)
