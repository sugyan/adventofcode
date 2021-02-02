import unittest
from typing import List

from day23 import Solution


class TestDay23(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return ["389125467"]

    def test_part_1(self) -> None:
        self.assertEqual(67_384_529, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(149_245_887_792, Solution(self.example_inputs()).part_2())
