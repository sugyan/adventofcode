from io import StringIO
from typing import TextIO

from aoc2022.day13 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 13


def test_part2() -> None:
    assert Solution(example_input()).part2() == 140
