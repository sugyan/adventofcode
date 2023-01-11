import sys
from collections import defaultdict, deque
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        grid = [list(line.strip()) for line in io]
        d: deque[tuple[tuple[int, int], int]] = deque()
        heightmap = {}
        min_steps = {}
        for i, row in enumerate(grid):
            for j, c in enumerate(row):
                heightmap[i, j] = ord({"S": "a", "E": "z"}.get(c, c))
                if c == "E":
                    d.append(((i, j), 0))
        while d:
            (i, j), steps = d.popleft()
            if (i, j) in min_steps:
                continue
            min_steps[i, j] = steps
            for p in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
                if heightmap.get(p, 0) >= heightmap[i, j] - 1:
                    d.append((p, steps + 1))
        dd = defaultdict(list)
        for (i, j), steps in min_steps.items():
            dd[grid[i][j]].append(steps)
        self.min_steps = {k: min(v) for k, v in dd.items()}

    def part1(self) -> int:
        return self.min_steps["S"]

    def part2(self) -> int:
        return min(self.min_steps[c] for c in ["S", "a"])


if __name__ == "__main__":
    run(Solution(sys.stdin))
