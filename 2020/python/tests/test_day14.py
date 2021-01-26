import unittest

from day14 import Solution


class TestDay14(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(
            165,
            Solution(
                """\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0""".splitlines()
            ).part_1(),
        )

    def test_part_2(self) -> None:
        self.assertEqual(
            208,
            Solution(
                """\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1""".splitlines()
            ).part_2(),
        )
