import unittest

from day15 import Solution


class TestDay15(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(436, Solution(["0,3,6"]).part_1())
        self.assertEqual(1, Solution(["1,3,2"]).part_1())
        self.assertEqual(10, Solution(["2,1,3"]).part_1())
        self.assertEqual(27, Solution(["1,2,3"]).part_1())
        self.assertEqual(78, Solution(["2,3,1"]).part_1())
        self.assertEqual(438, Solution(["3,2,1"]).part_1())
        self.assertEqual(1836, Solution(["3,1,2"]).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(175_594, Solution(["0,3,6"]).part_2())
        # self.assertEqual(2578, Solution(["1,3,2"]).part_2())
        # self.assertEqual(3_544_142, Solution(["2,1,3"]).part_2())
        # self.assertEqual(261_214, Solution(["1,2,3"]).part_2())
        # self.assertEqual(6_895_259, Solution(["2,3,1"]).part_2())
        # self.assertEqual(18, Solution(["3,2,1"]).part_2())
        # self.assertEqual(362, Solution(["3,1,2"]).part_2())
