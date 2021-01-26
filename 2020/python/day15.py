import sys
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.numbers = [int(x) for x in inputs[0].split(",")]

    def part_1(self) -> int:
        return self.__play(2020)

    def part_2(self) -> int:
        return self.__play(30_000_000)

    def __play(self, turns: int) -> int:
        memory = [0] * turns
        for i, number in enumerate(self.numbers):
            memory[number] = i + 1
        prev = self.numbers[-1]
        for i in range(len(self.numbers), turns):
            last = memory[prev]
            memory[prev] = i
            prev = 0 if last == 0 else i - last
        return prev


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
