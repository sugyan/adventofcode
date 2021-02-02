import unittest
from typing import List

from day22 import Solution


class TestDay22(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(306, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(291, Solution(self.example_inputs()).part_2())
