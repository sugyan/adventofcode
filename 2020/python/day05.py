import sys
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def convert(seat: str) -> int:
            return int(seat.translate(str.maketrans("BRFL", "1100")), 2)

        self.seats = list(map(convert, inputs))

    def part_1(self) -> int:
        return max(self.seats)

    def part_2(self) -> int:
        s = set(self.seats)
        for seat in range(min(s), max(s) + 1):
            if seat not in s:
                return seat
        raise ValueError


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
