import sys
from copy import deepcopy
from typing import Any, Dict, Iterable, List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty() -> Iterable[List[str]]:
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        self.rules: Dict[int, Any] = {}
        for i, lines in enumerate(split_at_empty()):
            if i == 0:
                for line in lines:
                    k, v = line.split(": ")
                    if '"' in v:
                        self.rules[int(k)] = v[1]
                    elif "|" in v:
                        self.rules[int(k)] = tuple(
                            [list(map(int, r.split(" "))) for r in v.split(" | ")]
                        )
                    else:
                        self.rules[int(k)] = list(map(int, v.split(" ")))
            if i == 1:
                self.messages = lines

    def part_1(self) -> int:
        return self.__validate(deepcopy(self.rules))

    def part_2(self) -> int:
        rules = deepcopy(self.rules)
        rules[8] = ([42], [42, 8])
        rules[11] = ([42, 31], [42, 11, 31])
        return self.__validate(rules)

    def __validate(self, rules: Dict[int, Any]) -> int:
        def match(s: str, rule: Any) -> List[str]:
            ret: List[str] = []
            if type(rule) == str:
                if s.startswith(rule):
                    ret.append(s[1:])
            if type(rule) == list:
                ret.append(s)
                for r in rule:
                    messages = []
                    for m in ret:
                        messages += match(m, rules[r])
                    ret = messages
            if type(rule) == tuple:
                for r in rule:
                    ret += match(s, r)
            return ret

        def validate(message: str) -> bool:
            return "" in match(message, rules[0])

        return len(list(filter(validate, self.messages)))


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
