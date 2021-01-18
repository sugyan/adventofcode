import sys
from functools import reduce
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty():
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        def convert(answers) -> int:
            return sum(map(lambda c: 1 << (ord(c) - ord("a")), answers))

        self.groups: List[List[int]] = []
        for lines in split_at_empty():
            self.groups.append([convert(line) for line in lines])

    def part_1(self) -> int:
        def count(answers: List[int]) -> int:
            return bin(reduce(lambda x, y: x | y, answers)).count("1")

        return sum(map(count, self.groups))

    def part_2(self) -> int:
        def count(answers: List[int]) -> int:
            return bin(reduce(lambda x, y: x & y, answers)).count("1")

        return sum(map(count, self.groups))


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
