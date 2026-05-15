from days.day01 import part1, part2

EXAMPLE = """\
3   4
4   3
2   5
1   3
3   9
3   3
"""


def test_part1():
    assert part1(EXAMPLE) == 11


def test_part2():
    assert part2(EXAMPLE) == 1
