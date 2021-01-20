import re
import sys
from typing import Dict, Iterable, List


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty() -> Iterable[List[str]]:
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        self.passports = []
        for lines in split_at_empty():
            fields = {}
            for line in lines:
                for field in line.split(" "):
                    k, v = field.split(":")
                    fields[k] = v
            self.passports.append(fields)

    def part_1(self) -> int:
        def validate(passport: Dict[str, str]) -> bool:
            return len(passport) == 8 or (len(passport) == 7 and "cid" not in passport)

        return len(list(filter(validate, self.passports)))

    def part_2(self) -> int:
        re_hgt = re.compile(r"(\d+)(cm|in)")
        re_hcl = re.compile(r"#[0-9a-f]{6}")
        re_ecl = re.compile(r"(?:amb|blu|brn|gry|grn|hzl|oth)")
        re_pid = re.compile(r"[0-9]{9}")

        def validate(passport: Dict[str, str]) -> bool:
            def validate_values() -> bool:
                for k, v in passport.items():
                    if k == "byr" and not 1920 <= int(v) <= 2002:
                        return False
                    if k == "iyr" and not 2010 <= int(v) <= 2020:
                        return False
                    if k == "eyr" and not 2020 <= int(v) <= 2030:
                        return False
                    if k == "hgt":
                        match = re_hgt.fullmatch(v)
                        if match:
                            unit = match.group(2)
                            if unit == "cm" and 150 <= int(match.group(1)) <= 193:
                                pass
                            elif unit == "in" and 59 <= int(match.group(1)) <= 76:
                                pass
                            else:
                                return False
                        else:
                            return False
                    if k == "hcl" and not re_hcl.fullmatch(v):
                        return False
                    if k == "ecl" and not re_ecl.fullmatch(v):
                        return False
                    if k == "pid" and not re_pid.fullmatch(v):
                        return False
                    if k == "cid":
                        pass
                return True

            return (
                len(passport) == 8 or (len(passport) == 7 and "cid" not in passport)
            ) and validate_values()

        return len(list(filter(validate, self.passports)))


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
