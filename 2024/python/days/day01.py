from collections import Counter

from rich import print


def part1(data: str) -> int:

    # MORE idiomatic way to write this according to LLM
    # [(x,y)] list of pairs
    pairs = [map(int, line.split()) for line in data.splitlines()]
    # I loose type info with the underying AI solution
    # list1, list2 = map(sorted, zip(*pairs))
    left, right = zip(*pairs)
    list1: list[int] = sorted(left)
    list2: list[int] = sorted(right)
    return sum(abs(a - b) for a, b in zip(list1, list2))

    # lines: list[str] = data.splitlines()
    # res = [0, 0]
    # list1: list[int] = []
    # list2: list[int] = []
    # for line in lines:
    #     left, right = line.split(None, 1)
    #     list1.append(int(left))
    #     list2.append(int(right))

    # list1.sort()
    # list2.sort()
    # temp = [abs(a - b) for a, b in zip(list1, list2)]

    # result = 0
    # for x in temp:
    #     result += x

    # return result


def part2(data: str) -> int:

    # encountered = {}

    pairs = [map(int, line.split()) for line in data.splitlines()]
    left, right = zip(*pairs)
    # list1: list[int] = sorted(left)
    # list2: list[int] = sorted(right)
    # dict {number : number_of_times_appears_in_collection}
    counts = Counter(right)
    return sum(n * counts[n] for n in left)
