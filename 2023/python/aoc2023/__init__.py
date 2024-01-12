from abc import ABC, abstractmethod
from typing import TextIO

Answer = int | str


class Solve(ABC):
    @abstractmethod
    def __init__(self, _: TextIO) -> None:
        pass

    @abstractmethod
    def part1(self) -> Answer:
        pass

    @abstractmethod
    def part2(self) -> Answer:
        pass
