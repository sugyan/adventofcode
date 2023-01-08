import sys
from typing import Iterator, TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.items = list(map(str.strip, io))

    def part1(self) -> int:
        def find_item(items: str) -> str:
            half = len(items) // 2
            return next(iter(set(items[:half]) & set(items[half:])))

        return sum(map(Solution.priority, map(find_item, self.items)))

    def part2(self) -> int:
        def groups() -> Iterator[list[str]]:
            for i in range(0, len(self.items), 3):
                yield self.items[i : i + 3]

        def find_item(group: list[str]) -> str:
            return next(iter(set(group[0]) & set(group[1]) & set(group[2])))

        return sum(map(Solution.priority, map(find_item, groups())))

    @staticmethod
    def priority(c: str) -> int:
        if c.islower():
            return ord(c) - ord("a") + 1
        else:
            return ord(c) - ord("A") + 27


if __name__ == "__main__":
    run(Solution(sys.stdin))
