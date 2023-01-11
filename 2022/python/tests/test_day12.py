from io import StringIO
from typing import TextIO

from aoc2022.day12 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 31


def test_part2() -> None:
    assert Solution(example_input()).part2() == 29
