from io import StringIO

from aoc2022.day01 import Solution


def example_input() -> StringIO:
    return StringIO(
        """\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 24000


def test_part2() -> None:
    assert Solution(example_input()).part2() == 45000
