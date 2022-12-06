from io import StringIO
from typing import TextIO

from aoc2022.day05 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""  # noqa W291
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == "CMZ"


def test_part2() -> None:
    assert Solution(example_input()).part2() == "MCD"
