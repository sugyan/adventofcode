import sys
from ast import literal_eval
from itertools import pairwise, product
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.cave = set()
        self.ymax = 0
        for line in map(str.strip, io):
            for (x0, y0), (x1, y1) in pairwise(map(literal_eval, line.split(" -> "))):
                xs = range(min(x0, x1), max(x0, x1) + 1)
                ys = range(min(y0, y1), max(y0, y1) + 1)
                self.cave |= set(product(xs, ys))
                self.ymax = max(self.ymax, y0, y1)

    def part1(self) -> int:
        return self.count_units(False)

    def part2(self) -> int:
        return self.count_units(True)

    def count_units(self, floor: bool) -> int:
        block = set(self.cave)
        stack = [(500, 0)]
        while stack:
            x, y = stack[-1]
            candidates = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
            if n := next((p for p in candidates if p not in block), None):
                if y < self.ymax + 1:
                    stack.append(n)
                    continue
                elif not floor:
                    break
            block.add(stack.pop())
        return len(block) - len(self.cave)


if __name__ == "__main__":
    run(Solution(sys.stdin))
