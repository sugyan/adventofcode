import unittest
from typing import List

from day13 import Solution


class TestDay13(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
939
7,13,x,x,59,x,31,19""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(295, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(1_068_781, Solution(self.example_inputs()).part_2())
        self.assertEqual(3417, Solution(["", "17,x,13,19"]).part_2())
        self.assertEqual(754_018, Solution(["", "67,7,59,61"]).part_2())
        self.assertEqual(779_210, Solution(["", "67,x,7,59,61"]).part_2())
        self.assertEqual(1_261_476, Solution(["", "67,7,x,59,61"]).part_2())
        self.assertEqual(1_202_161_486, Solution(["", "1789,37,47,1889"]).part_2())
