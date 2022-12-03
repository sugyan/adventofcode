from io import StringIO
from typing import TextIO

from aoc2022.day03 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 157


def test_part2() -> None:
    assert Solution(example_input()).part2() == 70
