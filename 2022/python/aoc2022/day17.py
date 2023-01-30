import sys
from collections import deque
from itertools import cycle
from typing import TextIO

from aoc2022 import Solve, run

ROCKS = [
    (0j, 1j, 2j, 3j),
    (1j, 1 + 0j, 1 + 1j, 1 + 2j, 2 + 1j),
    (0j, 1j, 2j, 1 + 2j, 2 + 2j),
    (0, 1, 2, 3),
    (0j, 1j, 1 + 0j, 1 + 1j),
]


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        self.jets = [{">": 1j, "<": -1j}[c] for c in io.read().strip()]

    def part1(self) -> int:
        return self.tower_height(2022)

    def part2(self) -> int:
        return self.tower_height(1_000_000_000_000)

    def tower_height(self, num_rocks: int) -> int:
        def empty(c: complex) -> bool:
            return c.real > 0 and 0 <= c.imag < 7 and c not in tower

        def check(pos: complex, d: complex, rock: tuple[complex, ...]) -> bool:
            return all(empty(pos + d + r) for r in rock)

        tower: set[complex] = set()
        cache: dict[tuple[int, int, frozenset[complex]], tuple[int, int]] = dict()
        rocks = cycle(enumerate(ROCKS))
        jets = cycle(enumerate(self.jets))
        top, i, j = 0, 0, 0
        for n in range(num_rocks):
            key = self.key(tower, top)
            if prev := cache.get((i, j, key)):
                d, m = divmod(num_rocks - n, n - prev[0])
                if m == 0:
                    return top + d * (top - prev[1])
            else:
                cache[(i, j, key)] = n, top

            pos = complex(top + 4, 2)
            i, rock = next(rocks)
            while True:
                j, jet = next(jets)
                if check(pos, jet, rock):
                    pos += jet
                if check(pos, -1, rock):
                    pos += -1
                else:
                    break
            tower |= {pos + r for r in rock}
            top = max(top, int(pos.real) + max(int(c.real) for c in rock))

        return top

    @staticmethod
    def key(tower: set[complex], top: int) -> frozenset[complex]:
        ret = set()
        q = deque([0 + 0j])
        while q:
            c = q.popleft()
            for d in [d for d in [(c - 1), (c - 1j), (c + 1j)] if d not in ret]:
                if d.imag in range(7) and -d.real <= top and top + 1 + d not in tower:
                    ret.add(d)
                    q.append(d)
        return frozenset(ret)


if __name__ == "__main__":
    run(Solution(sys.stdin))
