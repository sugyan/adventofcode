import re
import sys
from typing import List, Match, Optional, Pattern


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.lines: List[str] = inputs
        self.re: Pattern[str] = re.compile(r"(\d+)\-(\d+) (.): (.+)")

    def part_1(self) -> int:
        def validate(line: str) -> bool:
            match: Optional[Match[str]] = self.re.match(line)
            if match:
                lo: int = int(match.group(1))
                hi: int = int(match.group(2))
                c: str = match.group(3)
                password: str = match.group(4)
                return lo <= password.count(c) <= hi
            else:
                return False

        return len(list(filter(validate, self.lines)))

    def part_2(self) -> int:
        def validate(line: str) -> bool:
            match: Optional[Match[str]] = self.re.match(line)
            if match:
                p1: int = int(match.group(1)) - 1
                p2: int = int(match.group(2)) - 1
                c: str = match.group(3)
                password: str = match.group(4)
                return (password[p1] == c) ^ (password[p2] == c)
            else:
                return False

        return len(list(filter(validate, self.lines)))


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
