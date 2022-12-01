from abc import ABC, abstractmethod
from typing import Any, TextIO


class Solver(ABC):
    @abstractmethod
    def __init__(self, _: TextIO) -> None:
        pass

    @abstractmethod
    def part1(self) -> Any:
        pass

    @abstractmethod
    def part2(self) -> Any:
        pass
