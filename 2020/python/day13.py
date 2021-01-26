import sys
from itertools import count
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.lines = inputs

    def part_1(self) -> int:
        timestamp = int(self.lines[0])

        def wait(id: int) -> int:
            return id * (timestamp // id + 1) - timestamp

        min_id = min([int(x) for x in self.lines[1].split(",") if x != "x"], key=wait)
        return min_id * wait(min_id)

    def part_2(self) -> int:
        a, b = 1, 0
        for i, id_str in enumerate(self.lines[1].split(",")):
            if id_str != "x":
                bus_id = int(id_str)
                m = next(filter(lambda m: (a * m + b + i) % bus_id == 0, count()))
                a, b = a * bus_id, a * m + b
        return b


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
