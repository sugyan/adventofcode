import re
import sys
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        self.expressions = inputs
        self.re = re.compile(r"\(([^\(\)]*)\)")

    def part_1(self) -> int:
        return sum([self.evaluate(e, False) for e in self.expressions])

    def part_2(self) -> int:
        return sum([self.evaluate(e, True) for e in self.expressions])

    def evaluate(self, exp: str, adv: bool) -> int:
        while True:
            match = self.re.search(exp)
            if match:
                exp = self.re.sub(str(self.evaluate(match.group(1), adv)), exp, count=1)
            else:
                break
        parsed = ["*", *(map(str.strip, re.split(r"([\+\*])", exp)))]
        terms = [
            (parsed[x * 2], int(parsed[x * 2 + 1])) for x in range(len(parsed) // 2)
        ]
        ret = 1
        if adv:
            while terms:
                ope, val = terms.pop()
                if ope == "+":
                    terms[-1] = (terms[-1][0], terms[-1][1] + val)
                if ope == "*":
                    ret *= val
        else:
            for ope, val in terms:
                if ope == "+":
                    ret += val
                if ope == "*":
                    ret *= val
        return ret


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
