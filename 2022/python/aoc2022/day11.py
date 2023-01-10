import sys
from dataclasses import dataclass
from math import prod
from typing import Callable, TextIO

from aoc2022 import Solve, run


@dataclass
class Test:
    divisible: int
    throw_to: tuple[int, int]


@dataclass
class Monkey:
    starting_items: list[int]
    operation: Callable[[int], int]
    test: Test


class Solution(Solve):
    def __init__(self, io: TextIO) -> None:
        def parse(lines: list[str]) -> Monkey:
            return Monkey(
                starting_items=list(map(int, lines[1][18:].split(", "))),
                operation=eval(f"lambda old: {lines[2][19:]}"),
                test=Test(
                    divisible=int(lines[3][21:]),
                    throw_to=(int(lines[5][30:]), int(lines[4][29:])),
                ),
            )

        self.monkeys = list(map(parse, map(str.splitlines, io.read().split("\n\n"))))

    def part1(self) -> int:
        return self.monkey_business(20, 3)

    def part2(self) -> int:
        return self.monkey_business(10000, 1)

    def monkey_business(self, round: int, divide: int) -> int:
        items = list(map(lambda m: m.starting_items[:], self.monkeys))
        lcm = prod(m.test.divisible for m in self.monkeys)
        inspected = [0] * len(self.monkeys)
        for _ in range(round):
            for i, monkey in enumerate(self.monkeys):
                while items[i]:
                    level = monkey.operation(items[i].pop()) // divide
                    to = monkey.test.throw_to[level % monkey.test.divisible == 0]
                    items[to].append(level % lcm)
                    inspected[i] += 1
        return prod(sorted(inspected)[-2:])


if __name__ == "__main__":
    run(Solution(sys.stdin))
