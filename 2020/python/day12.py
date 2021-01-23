import sys
from enum import Enum
from typing import List, Tuple


class Action(Enum):
    N = "N"
    S = "S"
    E = "E"
    W = "W"
    L = "L"
    R = "R"
    F = "F"


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def parse(line: str) -> Tuple[Action, int]:
            action, value = line[0], line[1:]
            return Action(action), int(value)

        self.insructions = list(map(parse, inputs))

    def part_1(self) -> int:
        return sum([abs(v) for v in self.__navigate((1, 0), False)])

    def part_2(self) -> int:
        return sum([abs(v) for v in self.__navigate((10, 1), True)])

    def __navigate(self, d: Tuple[int, int], waypoint: bool) -> Tuple[int, int]:
        p = [0, 0]
        w = list(d)
        for action, value in self.insructions:
            if action == Action.N:
                if waypoint:
                    w[1] += value
                else:
                    p[1] += value
            if action == Action.S:
                if waypoint:
                    w[1] -= value
                else:
                    p[1] -= value
            if action == Action.E:
                if waypoint:
                    w[0] += value
                else:
                    p[0] += value
            if action == Action.W:
                if waypoint:
                    w[0] -= value
                else:
                    p[0] -= value
            if action == Action.L:
                for _ in range(value // 90):
                    w = [-w[1], w[0]]
            if action == Action.R:
                for _ in range(value // 90):
                    w = [w[1], -w[0]]
            if action == Action.F:
                p[0] += w[0] * value
                p[1] += w[1] * value
        return (p[0], p[1])


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
