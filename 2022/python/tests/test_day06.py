from io import StringIO

from aoc2022.day06 import Solution


def test_part1() -> None:
    assert Solution(StringIO("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).part1() == 7
    assert Solution(StringIO("bvwbjplbgvbhsrlpgdmjqwftvncz")).part1() == 5
    assert Solution(StringIO("nppdvjthqldpwncqszvftbrmjlhg")).part1() == 6
    assert Solution(StringIO("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).part1() == 10
    assert Solution(StringIO("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).part1() == 11


def test_part2() -> None:
    assert Solution(StringIO("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).part2() == 19
    assert Solution(StringIO("bvwbjplbgvbhsrlpgdmjqwftvncz")).part2() == 23
    assert Solution(StringIO("nppdvjthqldpwncqszvftbrmjlhg")).part2() == 23
    assert Solution(StringIO("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).part2() == 29
    assert Solution(StringIO("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).part2() == 26
