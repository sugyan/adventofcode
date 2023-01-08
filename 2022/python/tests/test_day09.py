from io import StringIO
from typing import TextIO

from aoc2022.day09 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"""
    )


def example_input_large() -> TextIO:
    return StringIO(
        """\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 13


def test_part2() -> None:
    assert Solution(example_input()).part2() == 1
    assert Solution(example_input_large()).part2() == 36
