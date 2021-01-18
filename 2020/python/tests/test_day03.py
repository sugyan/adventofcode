import unittest
from typing import List

from day03 import Solution


class TestDay03(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#""".splitlines()

    def test_part_1(self):
        self.assertEqual(7, Solution(self.example_inputs()).part_1())

    def test_part_2(self):
        self.assertEqual(336, Solution(self.example_inputs()).part_2())
