import sys
from typing import TextIO
from aoc2023 import Solve


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def match_count(s: str) -> int:
            winning, have = map(str.split, s.split(":")[1].split("|"))
            return len(set(winning) & set(have))

        self.matches = list(map(match_count, io))

    def part1(self) -> int:
        return sum(1 << i >> 1 for i in self.matches)

    def part2(self) -> int:
        cards = [1] * len(self.matches)
        for i, m in enumerate(self.matches):
            for j in range(m):
                cards[i + 1 + j] += cards[i]
        return sum(cards)


if __name__ == "__main__":
    solution = Solution(sys.stdin)
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
