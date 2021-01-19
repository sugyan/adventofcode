import unittest
from typing import List

from day08 import Solution


class TestDay08(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(5, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(8, Solution(self.example_inputs()).part_2())
