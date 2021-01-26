import unittest

from day16 import Solution


class TestDay16(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(
            71,
            Solution(
                """\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12""".splitlines()
            ).part_1(),
        )

    def test_part_2(self) -> None:
        self.assertEqual(
            ["row", "class", "seat"],
            Solution(
                """\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9""".splitlines()
            ).identify(),
        )
