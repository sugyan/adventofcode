import sys
from copy import deepcopy
from itertools import product, repeat
from operator import add
from typing import List, Set, Tuple


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.active: Set[Tuple[int, ...]] = set()
        for i, row in enumerate(inputs):
            for j, c in enumerate(row):
                if c == "#":
                    self.active.add((i, j, 0, 0))

    def part_1(self) -> int:
        return self.__simulate(list(filter(any, product(*repeat([-1, 0, 1], 3), [0]))))

    def part_2(self) -> int:
        return self.__simulate(list(filter(any, product(*repeat([-1, 0, 1], 4)))))

    def __simulate(self, neighbors: List[Tuple[int, ...]]) -> int:
        def activate(p: Tuple[int, ...]) -> bool:
            count = sum([tuple(map(add, p, d)) in active for d in neighbors])
            return count == 3 or (count == 2 and p in active)

        active = deepcopy(self.active)
        for _ in range(6):
            candidates = set(active)
            for p in active:
                candidates.update([(tuple(map(add, p, d))) for d in neighbors])
            active = set(filter(activate, candidates))
        return len(active)


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
