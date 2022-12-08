import sys
from typing import TextIO

from aoc2022 import Solve, run


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        parts = [s.splitlines() for s in io.read().split("\n\n")]
        self.stacks: list[list[str]] = [[] for _ in range((len(parts[0][0]) + 1) // 4)]
        for line in parts[0][-2::-1]:
            for i, c in enumerate(line[1::4]):
                if c.isalpha():
                    self.stacks[i].append(c)
        self.procedure = []
        for line in parts[1]:
            self.procedure.append(tuple(map(int, line.split(" ")[1::2])))

    def part1(self) -> str:
        return self.top_crates(True)

    def part2(self) -> str:
        return self.top_crates(False)

    def top_crates(self, reverse: bool) -> str:
        stacks = [s[:] for s in self.stacks]
        for m, f, t in self.procedure:
            stacks[t - 1] += stacks[f - 1][-m:][:: -1 if reverse else 1]
            del stacks[f - 1][-m:]
        return "".join([s[-1] for s in stacks])


if __name__ == "__main__":
    run(Solution(sys.stdin))
