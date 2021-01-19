import unittest
from typing import List

from day09 import Solution


class TestDay09(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(127, Solution(self.example_inputs(), preamble=5).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(62, Solution(self.example_inputs(), preamble=5).part_2())
