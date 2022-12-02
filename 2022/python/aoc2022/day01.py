import sys
from typing import TextIO

from aoc2022 import Solver


class Solution(Solver):
    def __init__(self, io: TextIO) -> None:
        self.sorted_calories = sorted(
            [sum(map(int, lines.splitlines())) for lines in io.read().split("\n\n")]
        )

    def part1(self) -> int:
        return self.sum_top_n(1)

    def part2(self) -> int:
        return self.sum_top_n(3)

    def sum_top_n(self, n: int) -> int:
        return sum(self.sorted_calories[-n:])


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
