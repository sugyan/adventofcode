from io import StringIO
from typing import TextIO

from aoc2022.day17 import Solution


def example_input() -> TextIO:
    return StringIO(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")


def test_part1() -> None:
    assert Solution(example_input()).part1() == 3068


def test_part2() -> None:
    assert Solution(example_input()).part2() == 1_514_285_714_288
