import unittest
from typing import List

from day17 import Solution


class TestDay17(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
.#.
..#
###""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(112, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(848, Solution(self.example_inputs()).part_2())
