import unittest
from typing import List

from day12 import Solution


class TestDay12(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
F10
N3
F7
R90
F11""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(25, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(286, Solution(self.example_inputs()).part_2())
