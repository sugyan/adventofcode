import unittest
from typing import List

from day02 import Solution


class TestDay02(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return '''\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc'''.splitlines()

    def test_part_1(self):
        solution = Solution(self.example_inputs())
        self.assertEqual(2, solution.part_1())

    def test_part_2(self):
        solution = Solution(self.example_inputs())
        self.assertEqual(1, solution.part_2())
