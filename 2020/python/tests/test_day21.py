import unittest
from typing import List

from day21 import Solution


class TestDay21(unittest.TestCase):
    def example_inputs(self) -> List[str]:
        return """\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(5, Solution(self.example_inputs()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(
            "mxmxvkd,sqjhc,fvjkl", Solution(self.example_inputs()).part_2()
        )
