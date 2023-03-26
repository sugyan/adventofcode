from io import StringIO
from typing import TextIO

from aoc2022.day18 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 64


def test_part2() -> None:
    assert Solution(example_input()).part2() == 58
