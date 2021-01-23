import unittest
from typing import List

from day11 import Solution


class TestDay11(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(37, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(26, Solution(self.example_inputs()).part_2())
