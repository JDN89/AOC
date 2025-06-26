use aoc_2024::days::{day01, day02}; // import the day01 module
use aoc_2024::util::read_input;

#[test]
fn day1() {
    let input = read_input("inputs/test_input_day01.txt");

    assert_eq!(day01::part1(&input), 11);
    assert_eq!(day01::part2(&input), 31);
}

#[test]
fn day2() {
    let input = read_input("inputs/test_input_day02.txt");

    // assert_eq!(day02::part1(&input), 2);
    assert_eq!(day02::part2(&input), 4);
}
