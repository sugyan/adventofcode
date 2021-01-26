import re
import sys
from abc import ABC, abstractmethod
from collections import defaultdict
from typing import Dict, List


class Decoder(ABC):
    def __init__(self) -> None:
        self.mem: Dict[int, int] = defaultdict(int)

    @abstractmethod
    def set_mask(self, mask: str) -> None:
        pass

    @abstractmethod
    def write(self, address: int, value: int) -> None:
        pass


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.inputs = inputs

    def part_1(self) -> int:
        class V1Decoder(Decoder):
            def set_mask(self, mask: str) -> None:
                self.masks = [(1 << 36) - 1, 0]
                for i, c in enumerate(reversed(mask)):
                    if c == "0":
                        self.masks[0] &= ~(1 << i)
                    if c == "1":
                        self.masks[1] |= 1 << i

            def write(self, address: int, value: int) -> None:
                self.mem[address] = value & self.masks[0] | self.masks[1]

        return sum(self.__run(V1Decoder()).values())

    def part_2(self) -> int:
        class V2Decoder(Decoder):
            def set_mask(self, mask: str) -> None:
                self.floating: List[int] = []
                self.mask = 0
                for i, c in enumerate(reversed(mask)):
                    if c == "1":
                        self.mask |= 1 << i
                    if c == "X":
                        self.floating.append(i)

            def write(self, address: int, value: int) -> None:
                addresses = [address | self.mask]
                for i in self.floating:
                    addresses = sum(
                        [[a | 1 << i, a & ~(1 << i)] for a in addresses], []
                    )
                for address in addresses:
                    self.mem[address] = value

        return sum(self.__run(V2Decoder()).values())

    def __run(self, decoder: Decoder) -> Dict[int, int]:
        re_program = re.compile(r"mem\[(\d+)\] = (\d+)")
        for line in self.inputs:
            match = re_program.fullmatch(line)
            if match:
                decoder.write(int(match.group(1)), int(match.group(2)))
            elif line.startswith("mask = "):
                decoder.set_mask(line[7:])
        return decoder.mem


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
