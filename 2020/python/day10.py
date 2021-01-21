import sys
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.adaptors = [0, *sorted([int(x) for x in inputs])]

    def part_1(self) -> int:
        diff1, diff3 = 0, 0
        for i in range(len(self.adaptors) - 1):
            diff = self.adaptors[i + 1] - self.adaptors[i]
            if diff == 1:
                diff1 += 1
            if diff == 3:
                diff3 += 1
        return diff1 * (diff3 + 1)

    def part_2(self) -> int:
        dp = [0] * len(self.adaptors)
        dp[0] = 1
        for i in range(len(self.adaptors) - 1):
            for j in range(3):
                if i >= j and self.adaptors[i + 1] - self.adaptors[i - j] <= 3:
                    dp[i + 1] += dp[i - j]
        return dp[-1]


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
