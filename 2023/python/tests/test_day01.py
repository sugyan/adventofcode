from io import StringIO

from aoc2023.day01 import Solution


def test_part1() -> None:
    assert (
        Solution(
            StringIO(
                """\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""
            )
        ).part1()
        == 142
    )


def test_part2() -> None:
    assert (
        Solution(
            StringIO(
                """\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""
            )
        ).part2()
        == 281
    )
