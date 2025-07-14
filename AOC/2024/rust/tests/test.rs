use aoc_2024::days::{day01, day02, day03, day04, day05}; // import the day01 module
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

#[test]
fn day3() {
    let input = read_input("inputs/test_input_day03.txt");
    // assert_eq!(day03::part1(&input), 161);
    assert_eq!(day03::part2(&input), 48);
}

#[test]
fn day4() {
    let input = read_input("inputs/test_input_day04.txt");
    // assert_eq!(day04::part1(&input), 18);
    assert_eq!(day04::part2(&input), 9);
}
#[test]
fn day5() {
    let input = read_input("inputs/test_input_day05.txt");
    assert_eq!(day05::part1(&input), 143);
    // assert_eq!(day04::part2(&input), 9);
}
