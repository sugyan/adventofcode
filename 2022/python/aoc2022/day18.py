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
            [c for c in chain(*map(self.adjacents, self.cubes)) if c not in self.cubes]
        )

    def part2(self) -> int:
        maxs = [max(map(lambda c: c[i], self.cubes)) + 1 for i in range(3)]
        seen: set[coord] = set()
        d = deque([(-1, -1, -1)])
        while d:
            c = d.popleft()
            for n in self.adjacents(c) - seen - self.cubes:
                if all([-1 <= n[i] <= maxs[i] for i in range(3)]):
                    seen.add(n)
                    d.append(n)
        return sum(c in seen for c in chain(*map(self.adjacents, self.cubes)))

    @staticmethod
    def adjacents(c: coord) -> set[coord]:
        return {
            (c[0] - 1, c[1], c[2]),
            (c[0] + 1, c[1], c[2]),
            (c[0], c[1] - 1, c[2]),
            (c[0], c[1] + 1, c[2]),
            (c[0], c[1], c[2] - 1),
            (c[0], c[1], c[2] + 1),
        }


if __name__ == "__main__":
    run(Solution(sys.stdin))
