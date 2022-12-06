import sys
from typing import TextIO

from aoc2022 import Solver


class Solution(Solver):
    def __init__(self, io: TextIO) -> None:
        self.data = io.read().strip()

    def part1(self) -> int:
        return self.marker_detected_position(4)

    def part2(self) -> int:
        return self.marker_detected_position(14)

    def marker_detected_position(self, window_size: int) -> int:
        for i in range(window_size, len(self.data) + 1):
            if len(set(self.data[i - window_size : i])) == window_size:
                return i
        raise ValueError("unreachable!")


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
