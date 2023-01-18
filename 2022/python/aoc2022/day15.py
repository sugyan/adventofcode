import re
import sys
from collections import Counter
from dataclasses import dataclass
from itertools import product
from typing import TextIO

from aoc2022 import Solve, run


@dataclass
class Report:
    @dataclass
    class Coordinate:
        x: int
        y: int

    sensor: Coordinate
    beacon: Coordinate
    distance: int

    def __init__(self, sx: int, sy: int, bx: int, by: int) -> None:
        self.sensor = self.Coordinate(sx, sy)
        self.beacon = self.Coordinate(bx, by)
        self.distance = abs(sx - bx) + abs(sy - by)


class Solution(Solve):
    MAX = 4_000_000

    def __init__(self, io: TextIO) -> None:
        def parse(line: str) -> Report:
            return Report(*map(int, re.findall(r"[xy]=(-?\d+)", line)))

        self.reports = list(map(parse, io))

    def part1(self) -> int:
        xs = set([r.beacon.x for r in self.reports if r.beacon.y == self.MAX // 2])
        return sum(map(lambda r: r[1] - r[0] + 1, self.ranges(self.MAX // 2))) - len(xs)

    def part2(self) -> int:
        ps, ns = [], []
        for r in self.reports:
            ps.append(r.sensor.y - r.sensor.x + (r.distance + 1))
            ps.append(r.sensor.y - r.sensor.x - (r.distance + 1))
            ns.append(r.sensor.y + r.sensor.x + (r.distance + 1))
            ns.append(r.sensor.y + r.sensor.x - (r.distance + 1))
        for y in set(
            (b0 + b1) // 2
            for b0, b1 in product(
                (b for b, n in Counter(ps).items() if n > 1),
                (b for b, n in Counter(ns).items() if n > 1),
            )
        ):
            ranges = self.ranges(y)
            if len(ranges) > 1:
                return (ranges[0][1] + 1) * 4_000_000 + y
        raise ValueError("unreachable!")

    def ranges(self, y: int) -> list[tuple[int, int]]:
        def f(report: Report) -> tuple[int, int]:
            return report.sensor.x, report.distance - abs(report.sensor.y - y)

        ranges: list[tuple[int, int]] = []
        for (xmin, xmax) in sorted(
            (x - r, x + r) for x, r in map(f, self.reports) if r >= 0
        ):
            if not ranges or xmin > ranges[-1][1] + 1:
                ranges.append((xmin, xmax))
            ranges[-1] = (ranges[-1][0], max(ranges[-1][1], xmax))
        return ranges


if __name__ == "__main__":
    run(Solution(sys.stdin))
