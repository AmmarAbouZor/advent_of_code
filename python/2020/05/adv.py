SPLIT_IDX = 7
ROWS_MAX = 127
COLUMNS_MAX = 7


def get_input_lines() -> list[str]:
    with open("input.txt") as f:
        return f.read().splitlines()


def calc_code_score(code: str) -> int:
    rows, cols = code[:SPLIT_IDX], code[SPLIT_IDX:]
    # Rows Calculations
    min, max = 0, ROWS_MAX
    for ch in rows:
        row_mid = min + (max - min) // 2
        if ch == "F":
            max = row_mid
        else:
            min = row_mid + 1

    row = min if rows[-1] == "F" else max

    # Cols Calculations
    min, max = 0, COLUMNS_MAX
    for ch in cols:
        col_mid = min + (max - min) // 2
        if ch == "L":
            max = col_mid
        else:
            min = col_mid + 1

    col = min if cols[-1] == "L" else max

    return row * 8 + col


def part_1():
    input = get_input_lines()
    answer = max((calc_code_score(line) for line in input))

    print(f"Part 1 answer is {answer}")


def test():
    test_input = {
        "FBFBBFFRLR": 357,
        "BFFFBBFRRR": 567,
        "FFFBBBFRRR": 119,
        "BBFFBBFRLL": 820,
    }

    for code, score in test_input.items():
        res_score = calc_code_score(code)
        assert score == res_score, f"Left: {score}, Right: {res_score}"

    print("Tests Passed")


test()
part_1()
