import unittest
from typing import List

from day01 import Solution


class TestDay01(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
1721
979
366
299
675
1456""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(514_579, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(241_861_950, Solution(self.example_inputs()).part_2())
