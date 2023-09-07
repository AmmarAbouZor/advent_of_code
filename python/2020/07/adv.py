def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def parse_input(input: str) -> dict[str, str]:
    map = {}
    for line in input.splitlines():
        parts = line.split("contain ")
        key = parts[0].replace(" bags", "")
        values = []
        if parts[1] != "no other bags.":
            for part in parts[1].split(", "):
                words = part.split(" ")
                values.append(f"{words[1]} {words[2]}")

        map[key] = values

    return map


def part_1():
    input = get_input()
    map = parse_input(input)
    print(map)


TEST_INPUT = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""


def test():
    map = parse_input(TEST_INPUT)
    print(map)


test()
