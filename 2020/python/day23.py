import sys
from functools import reduce
from typing import List, Tuple


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.cups = list(map(int, inputs[0]))

    def part_1(self) -> int:
        next_map = self.__simulate(len(self.cups), 100)

        def labels(curr: Tuple[int, int], _: int) -> Tuple[int, int]:
            cup = next_map[curr[1]]
            return (curr[0] * 10 + cup, cup)

        return reduce(labels, range(8), (0, 1))[0]

    def part_2(self) -> int:
        next_map = self.__simulate(1_000_000, 10_000_000)

        def labels_product(curr: Tuple[int, int], _: int) -> Tuple[int, int]:
            cup = next_map[curr[1]]
            return (curr[0] * cup, cup)

        return reduce(labels_product, range(2), (1, 1))[0]

    def __simulate(self, cups: int, moves: int) -> List[int]:
        ret = [0] * (cups + 1)
        for i, cup in enumerate(self.cups):
            if i > 0:
                ret[self.cups[i - 1]] = cup
            last = cup
        for i in range(len(self.cups), cups):
            ret[last] = last = i + 1
        ret[last] = self.cups[0]

        curr = self.cups[0]
        for _ in range(moves):
            p = curr
            pickups = []
            for _ in range(3):
                p = ret[p]
                pickups.append(p)
            dest = (curr - 2) % cups + 1
            while dest in pickups:
                dest = (dest - 2) % cups + 1
            ret[curr], ret[p], ret[dest] = ret[p], ret[dest], ret[curr]
            curr = ret[curr]
        return ret


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
