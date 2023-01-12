import sys
from ast import literal_eval
from functools import cmp_to_key
from math import prod
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.pairs = [
            tuple(map(literal_eval, lines))
            for lines in map(str.splitlines, io.read().split("\n\n"))
        ]

    def part1(self) -> int:
        return sum(i for i, pair in enumerate(self.pairs, 1) if self.cmp(*pair) < 0)

    def part2(self) -> int:
        dividers = [[[2]], [[6]]]
        packets = sorted(sum(map(list, self.pairs), dividers), key=cmp_to_key(self.cmp))
        return prod([i for i, p in enumerate(packets, 1) if p in dividers])

    @staticmethod
    def cmp(lhs: object, rhs: object) -> int:
        match lhs, rhs:
            case int(l), int(r):
                return l - r
            case list(l), list(r):
                return next((c for c in map(Solution.cmp, l, r) if c), len(l) - len(r))
            case int(l), list(r):
                return Solution.cmp([l], r)
            case list(l), int(r):
                return Solution.cmp(l, [r])
            case _:
                raise ValueError("unreachable!")


if __name__ == "__main__":
    run(Solution(sys.stdin))
