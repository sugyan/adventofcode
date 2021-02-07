import sys
from functools import reduce
from typing import List


DIV = 20_201_227


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.card_key = int(inputs[0])
        self.door_key = int(inputs[1])

    def part_1(self) -> int:
        def loop_size(key: int) -> int:
            loop, value = 0, 1
            while value != key:
                value = (value * 7) % DIV
                loop += 1
            return loop

        card_loop = loop_size(self.card_key)
        door_loop = loop_size(self.door_key)
        assert card_loop != door_loop
        return reduce(lambda x, _: (x * self.door_key) % DIV, range(card_loop), 1)


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
