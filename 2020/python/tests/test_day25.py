import unittest

from day25 import Solution


class TestDay25(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(14_897_079, Solution(["5764801", "17807724"]).part_1())
