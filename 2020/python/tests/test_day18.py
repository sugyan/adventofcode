import unittest

from day18 import Solution


class TestDay18(unittest.TestCase):
    def test_part_1(self) -> None:
        self.assertEqual(71, Solution([]).evaluate("1 + 2 * 3 + 4 * 5 + 6", False))
        self.assertEqual(
            51, Solution([]).evaluate("1 + (2 * 3) + (4 * (5 + 6))", False)
        )
        self.assertEqual(26, Solution([]).evaluate("2 * 3 + (4 * 5)", False))
        self.assertEqual(
            437, Solution([]).evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", False)
        )
        self.assertEqual(
            12240,
            Solution([]).evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", False),
        )
        self.assertEqual(
            13632,
            Solution([]).evaluate(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", False
            ),
        )

    def test_part_2(self) -> None:
        self.assertEqual(231, Solution([]).evaluate("1 + 2 * 3 + 4 * 5 + 6", True))
        self.assertEqual(51, Solution([]).evaluate("1 + (2 * 3) + (4 * (5 + 6))", True))
        self.assertEqual(46, Solution([]).evaluate("2 * 3 + (4 * 5)", True))
        self.assertEqual(
            1445, Solution([]).evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", True)
        )
        self.assertEqual(
            669_060,
            Solution([]).evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", True),
        )
        self.assertEqual(
            23340,
            Solution([]).evaluate(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", True
            ),
        )
