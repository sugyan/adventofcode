import re
import sys
from collections import defaultdict, deque
from typing import List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        re_contain = re.compile(r"(.*?) bags contain (.*?).")
        self.rules = {}
        for line in inputs:
            match = re_contain.fullmatch(line)
            if not match:
                continue
            self.rules[match.group(1)] = match.group(2).split(", ")

    def part_1(self) -> int:
        dd = defaultdict(list)
        for bag, contents in self.rules.items():
            if contents == ["no other bags"]:
                continue
            for content in contents:
                color = content[content.find(" ") + 1 : content.rfind(" ")]
                dd[color].append(bag)
        s = set()
        dq = deque(["shiny gold"])
        while dq:
            color = dq.popleft()
            if color not in s:
                s.add(color)
                dq.extend(dd[color])
        return len(s) - 1

    def part_2(self) -> int:
        re_numcolor = re.compile(r"(\d+) (.*?) bags?")
        dd = defaultdict(list)
        for bag, contents in self.rules.items():
            for content in contents:
                match = re_numcolor.fullmatch(content)
                if match:
                    dd[bag].append((int(match.group(1)), match.group(2)))
        ret = 0
        dq = deque([(1, "shiny gold")])
        while dq:
            num, color = dq.popleft()
            ret += num
            dq.extend([(num * e[0], e[1]) for e in dd[color]])
        return ret - 1


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
