from collections import Counter
from functools import reduce
from math import prod
from operator import or_
import re
import sys
from typing import TextIO
from aoc2023 import Solve


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def parse(line: str) -> list[Counter[str]]:
            return [
                Counter({c: int(n) for n, c in re.findall(r"(\d+) (\w)", s)})
                for s in line.split(":")[1].split(";")
            ]

        self.games = list(map(parse, io))

    def part1(self) -> int:
        bag = Counter({"r": 12, "g": 13, "b": 14})
        return sum(i for i, g in enumerate(self.games, 1) if all(s <= bag for s in g))

    def part2(self) -> int:
        return sum(prod(reduce(or_, g).values()) for g in self.games)


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
