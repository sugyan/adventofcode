import re
import sys
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.lines = inputs
        self.re = re.compile(r"(\d+)\-(\d+) (.): (.+)")

    def part_1(self) -> int:
        def validate(line: str) -> bool:
            match = self.re.match(line)
            if match:
                lo = int(match.group(1))
                hi = int(match.group(2))
                c = match.group(3)
                password = match.group(4)
                return lo <= password.count(c) <= hi
            else:
                return False

        return len(list(filter(validate, self.lines)))

    def part_2(self) -> int:
        def validate(line: str) -> bool:
            match = self.re.match(line)
            if match:
                p1 = int(match.group(1)) - 1
                p2 = int(match.group(2)) - 1
                c = match.group(3)
                password = match.group(4)
                return (password[p1] == c) ^ (password[p2] == c)
            else:
                return False

        return len(list(filter(validate, self.lines)))


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
