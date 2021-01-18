import unittest
from typing import List

from day06 import Solution


class TestDay06(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
abc

a
b
c

ab
ac

a
a
a
a

b""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(11, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(6, Solution(self.example_inputs()).part_2())
