import sys
from copy import deepcopy
from collections import deque
from typing import Deque, Iterable, List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty() -> Iterable[List[str]]:
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        self.decks = []
        for lines in split_at_empty():
            self.decks.append(deque(map(int, lines[1:])))

    def part_1(self) -> int:
        return sum(map(self.__score, self.__combat(deepcopy(self.decks), False)))

    def part_2(self) -> int:
        return sum(map(self.__score, self.__combat(deepcopy(self.decks), True)))

    def __combat(self, decks: List[Deque[int]], recursive: bool) -> List[Deque[int]]:
        memo = set()
        while all(decks):
            if recursive:
                key = tuple([tuple(deck) for deck in decks])
                if key in memo:
                    decks[1].clear()
                    return decks
                memo.add(key)
            cards = [deck.popleft() for deck in decks]
            if recursive and cards[0] <= len(decks[0]) and cards[1] <= len(decks[1]):
                results = self.__combat(
                    [
                        deque(list(decks[0])[: cards[0]]),
                        deque(list(decks[1])[: cards[1]]),
                    ],
                    True,
                )
                if results[0]:
                    decks[0].extend([cards[0], cards[1]])
                else:
                    decks[1].extend([cards[1], cards[0]])
            else:
                if cards[0] > cards[1]:
                    decks[0].extend([cards[0], cards[1]])
                else:
                    decks[1].extend([cards[1], cards[0]])
        return decks

    @staticmethod
    def __score(deck: Deque[int]) -> int:
        return sum([card * (len(deck) - i) for i, card in enumerate(deck)])


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
