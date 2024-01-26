from itertools import product
from math import prod
import re
import sys
from typing import TextIO
from aoc2023 import Solve


# Assume the following two things:
# - No number touches more than one symbol
# - No non-star symbol touches exactly two numbers
class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        schematic = list(map(str.strip, io))
        self.part_numbers: dict[tuple[int, int], list[int]] = {}
        for i, row in enumerate(schematic):
            for j, c in enumerate(row):
                if not c.isdecimal() and c != ".":
                    self.part_numbers[(i, j)] = []
        for i, row in enumerate(schematic):
            for m in re.finditer(r"(\d+)", row):
                ps = set(product((i - 1, i, i + 1), range(m.start() - 1, m.end() + 1)))
                for p in ps & self.part_numbers.keys():
                    self.part_numbers[p].append(int(m[1]))

    def part1(self) -> int:
        return sum(sum(v) for v in self.part_numbers.values())

    def part2(self) -> int:
        return sum(prod(v) for v in self.part_numbers.values() if len(v) == 2)


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
