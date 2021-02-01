import re
import sys
from collections import defaultdict
from typing import Dict, List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        re_contains = re.compile(r"^(.*?) \(contains (.*?)\)$")
        self.list = []
        for line in inputs:
            match = re_contains.fullmatch(line)
            if match:
                self.list.append(
                    (match.group(1).split(" "), match.group(2).split(", "))
                )

    def part_1(self) -> int:
        candidates = set(list(sum(self.__candidates().values(), [])))
        ret = 0
        for ingredients, _ in self.list:
            ret += len(list([x for x in ingredients if x not in candidates]))
        return ret

    def part_2(self) -> str:
        candidates = self.__candidates()
        dangerous_ingredients = {}
        while len(candidates):
            figure_outs = [
                (allergen, ingredients[0])
                for allergen, ingredients in candidates.items()
                if len(ingredients) == 1
            ]
            for figure_out in figure_outs:
                dangerous_ingredients[figure_out[0]] = figure_out[1]
                ingredient = candidates.pop(figure_out[0])[0]
                for ingredients in candidates.values():
                    if ingredient in ingredients:
                        ingredients.remove(ingredient)
        return ",".join(
            [dangerous_ingredients[key] for key in sorted(dangerous_ingredients.keys())]
        )

    def __candidates(self) -> Dict[str, List[str]]:
        counts_dict: Dict[str, Dict[str, int]] = defaultdict(lambda: defaultdict(int))
        for ingredients, allergens in self.list:
            for allergen in allergens:
                for ingredient in ingredients:
                    counts_dict[allergen][ingredient] += 1
        candidates = defaultdict(list)
        for allergen, counts in counts_dict.items():
            max_count = max(counts.values())
            for ingredient, count in counts.items():
                if count == max_count:
                    candidates[allergen].append(ingredient)
        return candidates


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
