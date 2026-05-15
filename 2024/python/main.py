import importlib
import sys
from pathlib import Path


def read_input(day: int) -> str:
    path = Path("inputs") / f"day{day:02}.txt"
    return path.read_text().strip()


def load_day(day: int):
    return importlib.import_module(f"days.day{day:02}")


def main():
    if len(sys.argv) < 2:
        print("Usage: python main.py <day>")
        sys.exit(1)

    day = int(sys.argv[1])

    module = load_day(day)
    data = read_input(day)

    print(f"Day {day:02}")
    print(f"Part 1: {module.part1(data)}")
    print(f"Part 2: {module.part2(data)}")


if __name__ == "__main__":
    main()
