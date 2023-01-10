import sys
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.values = [1]
        for line in map(str.strip, io):
            if line == "noop":
                self.values.append(self.values[-1])
            else:
                self.values.append(self.values[-1])
                self.values.append(self.values[-1] + int(line[5:]))

    def part1(self) -> int:
        return sum([self.values[i - 1] * i for i in [20, 60, 100, 140, 180, 220]])

    def part2(self) -> str:
        rows = []
        for values in [self.values[i * 40 : (i + 1) * 40] for i in range(6)]:
            rows.append("".join([".#"[abs(i - x) < 2] for i, x in enumerate(values)]))
        return "\n" + "\n".join(rows)


if __name__ == "__main__":
    run(Solution(sys.stdin))
