import sys
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def parse(line: str) -> tuple[complex, int]:
            return {"U": 1j, "D": -1j, "L": -1, "R": 1}[line[0]], int(line[2:])

        self.motions = list(map(parse, io))

    def part1(self) -> int:
        return self.tail_visited(2)

    def part2(self) -> int:
        return self.tail_visited(10)

    def tail_visited(self, knots_count: int) -> int:
        knots = [0j] * knots_count
        tails = set()
        for d, steps in self.motions:
            for _ in range(steps):
                knots[0] += d
                for i in range(1, knots_count):
                    diff = knots[i - 1] - knots[i]
                    if abs(diff) >= 2.0:
                        knots[i] += complex(
                            (diff.real > 0) - (diff.real < 0),
                            (diff.imag > 0) - (diff.imag < 0),
                        )
                tails.add(knots[-1])
        return len(tails)


if __name__ == "__main__":
    run(Solution(sys.stdin))
