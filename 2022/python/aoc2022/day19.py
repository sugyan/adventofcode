import sys
from dataclasses import dataclass
from math import prod
from typing import TextIO

from aoc2022 import Solve, run

Units = list[int]


@dataclass
class Blueprint:
    costs: tuple[Units, Units, Units, Units]
    maxs: list[int]

    def max_geodes(self, minutes: int) -> int:
        def dfs(robots: Units, resources: Units, minutes: int) -> int:
            ret = resources[3] + robots[3] * minutes
            for i in range(4):
                if (
                    i < 3 and resources[i] >= (self.maxs[i] - robots[i]) * minutes
                ) or any(self.costs[i][j] > 0 and robots[j] == 0 for j in range(4)):
                    continue
                wait = max(
                    max(0, (self.costs[i][j] - resources[j] - 1) // robots[j] + 1)
                    for j in range(4)
                    if robots[j] > 0
                )
                if wait >= minutes:
                    continue
                next_rob, next_res = robots[:], resources[:]
                for j in range(4):
                    next_res[j] += (wait + 1) * robots[j] - self.costs[i][j]
                next_rob[i] += 1
                ret = max(ret, dfs(next_rob, next_res, minutes - wait - 1))
            return ret

        return dfs([1, 0, 0, 0], [0, 0, 0, 0], minutes)


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
            maxs = [max(costs[j][i] for j in range(4)) for i in range(4)]
            return Blueprint(costs, maxs)

        self.blueprints = list(map(parse_blueprint, io))

    def part1(self) -> int:
        return sum(i * b.max_geodes(24) for i, b in enumerate(self.blueprints, 1))

    def part2(self) -> int:
        return prod(b.max_geodes(32) for b in self.blueprints[:3])


if __name__ == "__main__":
    run(Solution(sys.stdin))
