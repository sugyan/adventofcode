import sys
from itertools import combinations
from typing import List, Set


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.reports: List[int] = [int(i) for i in inputs]

    def part_1(self) -> int:
        s: Set[int] = set(self.reports)
        for report in self.reports:
            if 2020 - report in s:
                return report * (2020 - report)
        raise ValueError

    def part_2(self) -> int:
        s: Set[int] = set(self.reports)
        for reports in combinations(self.reports, 2):
            if 2020 - sum(reports) in s:
                return (2020 - sum(reports)) * reports[0] * reports[1]
        raise ValueError


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
