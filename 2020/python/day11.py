import sys
from collections import defaultdict
from copy import deepcopy
from typing import Dict, List, Optional, Tuple

Pos = Tuple[int, int]


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.layout = [list(x) for x in inputs]

    def part_1(self) -> int:
        return sum([s.count("#") for s in self.__simulate(True, 4)])

    def part_2(self) -> int:
        return sum([s.count("#") for s in self.__simulate(False, 5)])

    def __simulate(self, adjacent: bool, threshold: int) -> List[List[str]]:
        positions = self.__target_positions(adjacent)
        curr_state = deepcopy(self.layout)
        while True:
            next_state = deepcopy(curr_state)
            for i, row in enumerate(curr_state):
                for j, ch in enumerate(row):
                    if ch == ".":
                        continue
                    occupied = len(
                        [True for i, j in positions[(i, j)] if curr_state[i][j] == "#"]
                    )
                    if ch == "L" and occupied == 0:
                        next_state[i][j] = "#"
                    if ch == "#" and occupied >= threshold:
                        next_state[i][j] = "L"
            if next_state == curr_state:
                break
            else:
                curr_state = next_state
        return curr_state

    def __target_positions(self, adjacent: bool) -> Dict[Pos, List[Pos]]:
        def search_seat(i: int, j: int, di: int, dj: int) -> Optional[Pos]:
            while True:
                i += di
                j += dj
                if 0 <= i < len(self.layout) and 0 <= j < len(self.layout[i]):
                    if self.layout[i][j] != ".":
                        return (i, j)
                    elif adjacent:
                        return None
                else:
                    return None

        d = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        target_positions = defaultdict(list)
        for i in range(len(self.layout)):
            for j in range(len(self.layout[i])):
                for di, dj in d:
                    pos = search_seat(i, j, di, dj)
                    if pos:
                        target_positions[(i, j)].append(pos)
        return target_positions


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
