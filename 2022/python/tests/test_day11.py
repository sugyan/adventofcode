from io import StringIO
from typing import TextIO

from aoc2022.day11 import Solution


def example_input() -> TextIO:
    return StringIO(
        """\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"""
    )


def test_part1() -> None:
    assert Solution(example_input()).part1() == 10605


def test_part2() -> None:
    assert Solution(example_input()).part2() == 2_713_310_158
