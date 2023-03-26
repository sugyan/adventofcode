import sys
from ast import literal_eval
from collections import deque
from itertools import chain
from typing import TextIO

from aoc2022 import Solve, run

coord = tuple[int, int, int]


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.cubes: set[coord] = set(map(literal_eval, io))

    def part1(self) -> int:
        return len(
            [c for c in chain(*map(self.neighbors, self.cubes)) if c not in self.cubes]
        )

    def part2(self) -> int:
        maxs = [max(map(lambda c: c[i], self.cubes)) + 1 for i in range(3)]
        seen = set()
        d = deque([(-1, -1, -1)])
        count = 0
        while d:
            c = d.popleft()
            if c in seen:
                continue
            seen.add(c)
            for n in self.neighbors(c):
                if n in seen or not all([-1 <= n[i] <= maxs[i] for i in range(3)]):
                    continue
                if n in self.cubes:
                    count += 1
                else:
                    d.append(n)
        return count

    @staticmethod
    def neighbors(cube: coord) -> list[coord]:
        return [
            (cube[0] - 1, cube[1], cube[2]),
            (cube[0] + 1, cube[1], cube[2]),
            (cube[0], cube[1] - 1, cube[2]),
            (cube[0], cube[1] + 1, cube[2]),
            (cube[0], cube[1], cube[2] - 1),
            (cube[0], cube[1], cube[2] + 1),
        ]


if __name__ == "__main__":
    run(Solution(sys.stdin))
