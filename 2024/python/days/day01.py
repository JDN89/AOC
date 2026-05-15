from rich import print


def part1(data: str) -> int:
    lines: list[str] = data.splitlines()
    res = [0, 0]
    list1: list[int] = []
    list2: list[int] = []
    for line in lines:
        left, right = line.split(None, 1)
        list1.append(int(left))
        list2.append(int(right))

    list1.sort()
    list2.sort()
    temp = [abs(a - b) for a, b in zip(list1, list2)]

    result = 0
    for x in temp:
        result += x

    return result


def part2(data: str) -> int:

    return 1
