import sys
from typing import TextIO

from aoc2022 import Solver


class Solution(Solver):
    def __init__(self, io: TextIO) -> None:
        self.sorted_calories = sorted(
            [sum(map(int, lines.splitlines())) for lines in io.read().split("\n\n")]
        )

    def part1(self) -> int:
        return self.top_sum(1)

    def part2(self) -> int:
        return self.top_sum(3)

    def top_sum(self, n: int) -> int:
        return sum(self.sorted_calories[-n:])


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(solution.part1())
    print(solution.part2())
