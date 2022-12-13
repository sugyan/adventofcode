from io import StringIO
from typing import TextIO

from aoc2022.day08 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
30373
25512
65332
33549
35390
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 21


def test_part2() -> None:
    assert Solution(example_input()).part2() == 8
