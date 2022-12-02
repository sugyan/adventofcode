from io import StringIO
from typing import TextIO

from aoc2022.day02 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
A Y
B X
C Z
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 15


def test_part2() -> None:
    assert Solution(example_input()).part2() == 12
