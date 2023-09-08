TARGET_NAME = "shiny gold"


class Bag:
    def __init__(self, name: str, count: int) -> None:
        self.name = name
        self.count = count

    def __str__(self) -> str:
        return f"name: {self.name}, count: {self.count}"

    def __repr__(self) -> str:
        return f"(name: {self.name}, count: {self.count})"


def get_input() -> str:
    with open("input.txt") as f:
        return f.read()


def parse_input(input: str) -> dict[str, list[Bag]]:
    map = {}
    for line in input.splitlines():
        parts = line.split("contain ")
        key = parts[0].replace(" bags ", "")
        values = []
        if parts[1] != "no other bags.":
            for part in parts[1].split(", "):
                words = part.split(" ")
                name = f"{words[1]} {words[2]}"
                count = int(words[0])
                values.append(Bag(name, count))

        map[key] = values

    return map


def count_valid_bags(input: str) -> int:
    bags_map = parse_input(input)
    valid_bags = {TARGET_NAME}
    bags_updated = True

    while bags_updated:
        bags_updated = False
        for bag, children in bags_map.items():
            if bag in valid_bags:
                continue
            if any(child.name in valid_bags for child in children):
                bags_updated = True
                valid_bags.add(bag)

    return len(valid_bags) - 1


def count_gold_nested(input: str) -> int:
    bags_map = parse_input(input)
    bags_count = count_nested_bags(TARGET_NAME, bags_map)

    return bags_count


def count_nested_bags(name: str, bags_map: dict[str, list[Bag]]) -> int:
    # This method could be written like this
    # return sum(bag.count + bag.count * count_nested_bags(bag.name, bags_map) for bag in bags_map[name])
    count = 0
    for bag in bags_map[name]:
        count += bag.count + bag.count * count_nested_bags(bag.name, bags_map)

    return count


def part_1():
    input = get_input()
    valid_bags = count_valid_bags(input)
    print(f"Part 1 answer is {valid_bags}")


def part_2():
    input = get_input()
    bags_count = count_gold_nested(input)
    print(f"Part 2 answer is {bags_count}")


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
    bags_count = count_valid_bags(TEST_INPUT)
    assert 4 == bags_count, f"Part 1: Left 4, Right {bags_count}"
    nested_bags = count_gold_nested(TEST_INPUT)
    assert 32 == nested_bags, f"Part 2: Left 32, Right {nested_bags}"
    print("Test passed")


test()
part_1()
part_2()
