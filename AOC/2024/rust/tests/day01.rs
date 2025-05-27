use rust::days::{day01, day02}; // import the day01 module
use rust::util::read_input;

#[test]
fn test_day1_example() {
    let input = read_input("inputs/test_input_day01.txt");

    assert_eq!(day01::part1(&input), 11);
    assert_eq!(day01::part2(&input), 31);
}

#[test]
fn test_day2_example() {
    let input = read_input("inputs/test_input_day02.txt");

    assert_eq!(day02::part1(&input), 1);
    assert_eq!(day02::part2(&input), 2);
}
