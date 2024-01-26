from io import StringIO
from typing import TextIO


from aoc2023.day03 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 4361


def test_part2() -> None:
    assert Solution(example_input()).part2() == 467_835
