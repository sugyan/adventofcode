from io import StringIO
from typing import TextIO

from aoc2022.day04 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 2


def test_part2() -> None:
    assert Solution(example_input()).part2() == 4
