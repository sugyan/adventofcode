import sys
from dataclasses import dataclass
from math import prod
from typing import TextIO

from aoc2022 import Solve, run


@dataclass
class Blueprint:
    costs: tuple[list[int], ...]
    maxs: tuple[int, ...]

    def max_geodes(self, minutes: int) -> int:
        best = 0

        def dfs(robots: list[int], resources: list[int], minutes: int) -> None:
            nonlocal best

            geodes = resources[3] + robots[3] * minutes
            if geodes + (minutes - 1) * minutes // 2 < best:
                return

            best = max(best, geodes)
            for i, costs in enumerate(self.costs):
                if i < 3 and resources[i] >= (self.maxs[i] - robots[i]) * minutes:
                    continue
                if any(c > 0 and r == 0 for c, r in zip(costs, robots)):
                    continue
                waits = (
                    (costs[j] - resources[j] - 1) // robots[j] + 1
                    for j in range(4)
                    if robots[j] > 0 and costs[j] > resources[j]
                )
                wait = max(waits, default=0)
                if wait >= minutes:
                    continue
                next_state = robots[:], resources[:]
                for j in range(4):
                    next_state[0][i] += i == j
                    next_state[1][j] += (wait + 1) * robots[j] - costs[j]
                dfs(*next_state, minutes - wait - 1)

        dfs([1, 0, 0, 0], [0, 0, 0, 0], minutes)
        return best


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def parse_blueprint(line: str) -> Blueprint:
            v = list(map(int, filter(str.isdecimal, line.split(" "))))
            costs = (
                [v[0], 0, 0, 0],
                [v[1], 0, 0, 0],
                [v[2], v[3], 0, 0],
                [v[4], 0, v[5], 0],
            )
            maxs = (max(v[0], v[1], v[2], v[4]), v[3], v[5], 0)
            return Blueprint(costs, maxs)

        self.blueprints = list(map(parse_blueprint, io))

    def part1(self) -> int:
        return sum(i * b.max_geodes(24) for i, b in enumerate(self.blueprints, 1))

    def part2(self) -> int:
        return prod(b.max_geodes(32) for b in self.blueprints[:3])


if __name__ == "__main__":
    run(Solution(sys.stdin))
