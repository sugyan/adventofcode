import sys
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def parse(line: str) -> tuple[set[int], set[int]]:
            a0, a1 = map(lambda x: list(map(int, x.split("-"))), line.split(","))
            return (
                set(range(a0[0], a0[1] + 1)),
                set(range(a1[0], a1[1] + 1)),
            )

        self.pairs = list(map(parse, io))

    def part1(self) -> int:
        return len(list(filter(lambda a: a[0] <= a[1] or a[1] <= a[0], self.pairs)))

    def part2(self) -> int:
        return len(list(filter(lambda a: any(a[0] & a[1]), self.pairs)))


if __name__ == "__main__":
    run(Solution(sys.stdin))
