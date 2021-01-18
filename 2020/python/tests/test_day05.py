import unittest

from day05 import Solution


class TestDay05(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(
            820,
            Solution(
                """\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL""".splitlines()
            ).part_1(),
        )
