import unittest
from typing import List

from day10 import Solution


class TestDay10(unittest.TestCase):
    def example_inputs_1(self) -> List[str]:
        return """\
16
10
15
5
1
11
7
19
6
12
4""".splitlines()

    def example_inputs_2(self) -> List[str]:
        return """\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3""".splitlines()

    def test_part_1(self) -> None:
        self.assertEqual(35, Solution(self.example_inputs_1()).part_1())
        self.assertEqual(220, Solution(self.example_inputs_2()).part_1())

    def test_part_2(self) -> None:
        self.assertEqual(8, Solution(self.example_inputs_1()).part_2())
        self.assertEqual(19_208, Solution(self.example_inputs_2()).part_2())
