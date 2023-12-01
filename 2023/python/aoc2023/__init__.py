from typing import Any, Protocol, TextIO


class Solve(Protocol):
    def __init__(self, _: TextIO) -> None:
        pass

    def part1(self) -> Any:
        pass

    def part2(self) -> Any:
        pass
