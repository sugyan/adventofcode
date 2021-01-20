import sys
from enum import Enum
from typing import List, Optional, Tuple


class Operation(Enum):
    ACC = "acc"
    JMP = "jmp"
    NOP = "nop"


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def parse(line: str) -> Tuple[Operation, int]:
            ope, arg = line.split(" ")
            return Operation(ope), int(arg)

        self.instructions = list(map(parse, inputs))

    def part_1(self) -> int:
        return self.__run()[0]

    def part_2(self) -> int:
        for i in range(len(self.instructions)):
            ret = self.__run(i)
            if ret[1]:
                return ret[0]
        raise ValueError

    def __run(self, change: Optional[int] = None) -> Tuple[int, bool]:
        visited = set()
        i, acc = 0, 0
        while i < len(self.instructions):
            if i in visited:
                return acc, False
            visited.add(i)

            ope, arg = self.instructions[i]
            if i == change:
                if ope == Operation.ACC:
                    pass
                elif ope == Operation.JMP:
                    ope = Operation.NOP
                elif ope == Operation.NOP:
                    ope = Operation.JMP
            if ope == Operation.ACC:
                acc += arg
                i += 1
            if ope == Operation.JMP:
                i += arg
            if ope == Operation.NOP:
                i += 1
        return acc, True


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
