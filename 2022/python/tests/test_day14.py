from io import StringIO
from typing import TextIO

from aoc2022.day14 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 24


def test_part2() -> None:
    assert Solution(example_input()).part2() == 93
