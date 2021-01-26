import sys
from collections import defaultdict
from functools import reduce
from typing import Iterable, List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty() -> Iterable[List[str]]:
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        def parse_ticket(line: str) -> List[int]:
            return [int(x) for x in line.split(",")]

        for i, lines in enumerate(split_at_empty()):
            if i == 0:
                self.rules = []
                for line in lines:
                    k, v = line.split(": ")
                    ranges = []
                    for minmax in v.split(" or "):
                        ranges.append([int(m) for m in minmax.split("-")])
                    self.rules.append((k, ranges))
            if i == 1:
                self.ticket = parse_ticket(lines[1])
            if i == 2:
                self.nearby = [parse_ticket(line) for line in lines[1:]]

    def part_1(self) -> int:
        valid = set()
        for _, ranges in self.rules:
            for rmin, rmax in ranges:
                for i in range(rmin, rmax + 1):
                    valid.add(i)
        invalid = []
        for ticket in self.nearby:
            invalid.extend([val for val in ticket if val not in valid])
        return sum(invalid)

    def part_2(self) -> int:
        fields = self.identify()
        return reduce(
            lambda x, y: x * y,
            [v for v, f in zip(self.ticket, fields) if f.startswith("departure")],
        )

    def identify(self) -> List[str]:
        availables = defaultdict(set)
        for field, ranges in self.rules:
            for rmin, rmax in ranges:
                for i in range(rmin, rmax + 1):
                    availables[i].add(field)

        fields = [field for field, _ in self.rules]
        candidates = [set(fields) for _ in range(len(self.ticket))]
        for ticket in self.nearby:
            if any([not availables[val] for val in ticket]):
                continue
            for i, val in enumerate(ticket):
                candidates[i] &= availables[val]

        while any([len(c) > 1 for c in candidates]):
            for i, c1 in enumerate(candidates):
                if len(c1) == 1:
                    field = next(iter(c1))
                    for j, c2 in enumerate(candidates):
                        if j != i:
                            c2 -= set([field])
        return [next(iter(c)) for c in candidates]


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
