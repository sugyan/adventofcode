import sys
from collections import deque
from itertools import combinations
from typing import Deque, Iterable, List


class Solution:
    def __init__(self, inputs: List[str], preamble: int = 25) -> None:
        self.numbers = [int(x) for x in inputs]
        self.preamble = preamble

    def part_1(self) -> int:
        def has_pair(numbers: Iterable, target: int) -> bool:
            for c in combinations(dq, 2):
                if sum(c) == target:
                    return True
            return False

        dq: Deque[int] = deque(self.numbers[: self.preamble])
        for number in self.numbers[self.preamble :]:
            if not has_pair(dq, number):
                return number
            dq.popleft()
            dq.append(number)
        raise ValueError

    def part_2(self) -> int:
        target: int = self.part_1()
        lo, hi = (0, 0)
        total = self.numbers[0]
        while total != target:
            if total < target:
                hi += 1
                total += self.numbers[hi]
            else:
                total -= self.numbers[lo]
                lo += 1
        contiguous = self.numbers[lo : hi + 1]
        return min(contiguous) + max(contiguous)


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
