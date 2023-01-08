import sys
from itertools import chain
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def rotate90(grid: list[list[int]]) -> list[list[int]]:
            return list(map(list, zip(*grid[::-1])))

        grid = [list(map(int, s.strip())) for s in io]
        self.v = [[0 for _ in row] for row in grid]
        self.s = [[1 for _ in row] for row in grid]
        for _ in range(4):
            for i, row in enumerate(grid):
                for j, col in enumerate(row):
                    lower = [h < col for h in row[j + 1 :]]
                    self.v[i][j] |= all(lower)
                    self.s[i][j] *= len(lower) if all(lower) else lower.index(False) + 1
            grid, self.v, self.s = map(rotate90, [grid, self.v, self.s])

    def part1(self) -> int:
        return sum(chain(*self.v))

    def part2(self) -> int:
        return max(chain(*self.s))


if __name__ == "__main__":
    run(Solution(sys.stdin))
