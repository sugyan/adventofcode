import unittest
from typing import List

from day07 import Solution


class TestDay07(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(4, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(32, Solution(self.example_inputs()).part_2())
        self.assertEqual(
            126,
            Solution(
                """\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.""".splitlines()
            ).part_2(),
        )
