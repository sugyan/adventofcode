import sys
from functools import reduce
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.grid = inputs

    def part_1(self) -> int:
        return self.__count(3, 1)

    def part_2(self) -> int:
        return reduce(
            lambda x, y: x * y,
            [
                self.__count(1, 1),
                self.__count(3, 1),
                self.__count(5, 1),
                self.__count(7, 1),
                self.__count(1, 2),
            ],
        )

    def __count(self, right: int, down: int) -> int:
        ret = 0
        for j, i in enumerate(range(0, len(self.grid), down)):
            row = self.grid[i]
            if row[(j * right) % len(row)] == "#":
                ret += 1
        return ret


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
