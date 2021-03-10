import sys
from copy import deepcopy
from operator import add
from typing import List, Set, Tuple


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def position(s: str) -> Tuple[int, int]:
            p = [0, 0]
            ns = False
            for c in s:
                if c == "e":
                    p[0] += 1 if ns else 2
                    ns = False
                if c == "s":
                    p[1] -= 1
                    ns = True
                if c == "w":
                    p[0] -= 1 if ns else 2
                    ns = False
                if c == "n":
                    p[1] += 1
                    ns = True
            return p[0], p[1]

        self.flipped: Set[Tuple[int, int]] = set()
        for line in inputs:
            pos = position(line)
            if pos in self.flipped:
                self.flipped.remove(pos)
            else:
                self.flipped.add(pos)

    def part_1(self) -> int:
        return len(self.flipped)

    def part_2(self) -> int:
        neighbors = ((2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1))
        flipped = deepcopy(self.flipped)

        def flip(p: Tuple[int, int]) -> bool:
            count = sum([tuple(map(add, p, d)) in flipped for d in neighbors])
            return count == 2 or (count == 1 and p in flipped)

        for _ in range(100):
            candidates = set(flipped)
            for p in flipped:
                for d in neighbors:
                    candidates.add((p[0] + d[0], p[1] + d[1]))
            flipped = set(filter(flip, candidates))
        return len(flipped)


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
