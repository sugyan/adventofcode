from typing import Any, Protocol, TextIO


class Solve(Protocol):
    def __init__(self, _: TextIO) -> None:
        pass

    def part1(self) -> Any:
        pass

    def part2(self) -> Any:
        pass


def run(solution: Solve) -> None:
    print(f"Part 1: {solution.part1()}")
    print(f"Part 2: {solution.part2()}")
