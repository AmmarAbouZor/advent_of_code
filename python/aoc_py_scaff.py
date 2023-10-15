#!/usr/bin/python3

"""
This script make a small scaffolding for advent of code solutions in python3
The pattern should be like following: 
- For each day there is folder named after it
- In the folder there is two files: input.txt and adv.py
The scripts scans the current folder to figure out which day the next day should be

"""

import os
import sys

INPUT_FILE = "input.txt"
CODE_FILE = "adv.py"
TEST_FILE = "test.txt"
CODE_CONTENT = """def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def get_test_input() -> str:
    with open("test.txt") as f:
        return f.read()


def part_1():
    pass


def part_2():
    pass


def run_test():
    pass


if __name__ == "__main__":
    run_test()
    part_1()
    part_2()
"""


def main():
    folder_path = get_next_folder_path()
    os.mkdir(folder_path)

    input_file = os.path.join(folder_path, INPUT_FILE)
    open(input_file, "w").close()
    print(f"file: {input_file} has been created", file=sys.stderr)

    test_file = os.path.join(folder_path, TEST_FILE)
    open(test_file, "w").close()
    print(f"file: {test_file} has been created", file=sys.stderr)

    code_file = os.path.join(folder_path, CODE_FILE)
    with open(code_file, "w") as f:
        f.write(CODE_CONTENT)
    print(f"file: {code_file} has been created", file=sys.stderr)

    print(f"{test_file}")
    print(f"{input_file}")
    print(f"{code_file}")


def get_next_folder_path():
    cwd = os.getcwd()
    folders = [f.name for f in os.scandir(cwd) if f.is_dir() and f.name.isnumeric()]
    if len(folders) == 0:
        return "01"

    folders.sort()
    last_num = int(folders[-1])
    new_num = last_num + 1
    folder_name = f"{new_num:02d}"
    path = os.path.join(cwd, folder_name)

    return path


if __name__ == "__main__":
    main()
