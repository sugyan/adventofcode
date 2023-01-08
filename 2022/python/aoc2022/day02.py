import sys
from collections import Counter
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.counts = Counter(map(str.strip, io))

    def part1(self) -> int:
        return self.total_score(
            {
                "A X": 4,  # 1 + 3
                "A Y": 8,  # 2 + 6
                "A Z": 3,  # 3 + 0
                "B X": 1,  # 1 + 0
                "B Y": 5,  # 2 + 3
                "B Z": 9,  # 3 + 6
                "C X": 7,  # 1 + 6
                "C Y": 2,  # 2 + 0
                "C Z": 6,  # 3 + 3
            }
        )

    def part2(self) -> int:
        return self.total_score(
            {
                "A X": 3,  # 3 + 0
                "A Y": 4,  # 1 + 3
                "A Z": 8,  # 2 + 6
                "B X": 1,  # 1 + 0
                "B Y": 5,  # 2 + 3
                "B Z": 9,  # 3 + 6
                "C X": 2,  # 2 + 0
                "C Y": 6,  # 3 + 3
                "C Z": 7,  # 1 + 6
            }
        )

    def total_score(self, scores: dict[str, int]) -> int:
        return sum([v * scores.get(k, 0) for k, v in self.counts.items()])


if __name__ == "__main__":
    run(Solution(sys.stdin))
