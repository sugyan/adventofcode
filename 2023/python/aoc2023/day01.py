import sys
from typing import TextIO
from aoc2023 import Solve


class Solution(Solve):
    SPELLS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    def __init__(self, io: TextIO) -> None:
        self.input = list(map(str.strip, io.readlines()))

    def part1(self) -> int:
        return sum(map(Solution.calibration_value, self.input))

    def part2(self) -> int:
        def replace(s: str) -> str:
            for i, spell in enumerate(Solution.SPELLS, start=1):
                s = s.replace(spell, f"{spell}{i}{spell}")
            return s

        return sum(map(Solution.calibration_value, map(replace, self.input)))

    @staticmethod
    def calibration_value(s: str) -> int:
        digits = list(filter(str.isdecimal, s))
        return int(digits[0] + digits[-1])


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
