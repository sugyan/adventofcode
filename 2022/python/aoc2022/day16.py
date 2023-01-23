import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from typing import Generator, TextIO

from aoc2022 import Solve, run

RE_INPUT = re.compile(
    r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"
)


@dataclass
class Valve:
    label: str
    flow_rate: int
    tunnels: list[str]


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        valves = [
            Valve(
                label=m.group(1),
                flow_rate=int(m.group(2)),
                tunnels=m.group(3).split(", "),
            )
            for m in RE_INPUT.finditer(io.read())
        ]
        names = [v.label for v in valves]
        dists = {n: {n: len(valves) for n in names} for n in names}
        for v in valves:
            dists[v.label][v.label] = 0
            for t in v.tunnels:
                dists[v.label][t] = 1
        for k in names:
            for i in names:
                for j in names:
                    dists[i][j] = min(dists[i][j], dists[i][k] + dists[k][j])
        self.rates = {v.label: v.flow_rate for v in valves if v.flow_rate > 0}
        self.dists = dists

    def part1(self) -> int:
        return max(total for total, _ in self.totals("AA", frozenset(), 30, 0))

    def part2(self) -> int:
        max_totals: dict[frozenset[str], int] = defaultdict(int)
        for total, opened in self.totals("AA", frozenset(), 26, 0):
            max_totals[opened] = max(max_totals[opened], total)
        items = sorted(((v, k) for k, v in max_totals.items()), reverse=True)
        best = 0
        for t0, o0 in items:
            if t0 * 2 < best:
                break
            for t1, o1 in items:
                if not o0 & o1:
                    best = max(best, t0 + t1)
        return best

    def totals(
        self, src: str, opened: frozenset[str], minutes: int, total: int
    ) -> Generator[tuple[int, frozenset[str]], None, None]:
        for dst in self.rates.keys() - opened:
            if (remain := minutes - self.dists[src][dst] - 1) > 0:
                yield from self.totals(
                    dst, opened | {dst}, remain, total + self.rates[dst] * remain
                )
        yield total, opened


if __name__ == "__main__":
    run(Solution(sys.stdin))
