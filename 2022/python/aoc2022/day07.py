import sys
from collections import defaultdict
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.total_sizes: dict[tuple[str, ...], int] = defaultdict(int)
        curr: list[str] = []
        for line in io.readlines():
            parts = line.strip().split()
            if parts[:2] == ["$", "cd"]:
                if parts[2] == "..":
                    curr.pop()
                else:
                    curr.append(parts[2])
            if parts[0].isnumeric():
                for i in range(len(curr)):
                    self.total_sizes[tuple(curr[: i + 1])] += int(parts[0])

    def part1(self) -> int:
        return sum(filter(lambda x: x <= 100_000, self.total_sizes.values()))

    def part2(self) -> int:
        total = self.total_sizes[tuple(["/"])]
        return min(filter(lambda x: total - x < 40_000_000, self.total_sizes.values()))


if __name__ == "__main__":
    run(Solution(sys.stdin))
