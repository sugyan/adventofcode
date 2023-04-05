from io import StringIO
from typing import TextIO

from aoc2022.day19 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 33
