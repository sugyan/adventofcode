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
        solution = Solution(self.example_inputs())
        self.assertEqual(7, solution.part_1())

    def test_part_2(self):
        solution = Solution(self.example_inputs())
        self.assertEqual(336, solution.part_2())
