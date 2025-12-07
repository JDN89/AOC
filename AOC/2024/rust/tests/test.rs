use aoc_2024::days;
use std::fs; // Use the library crate's public module

fn read_test_input(day: &str) -> String {
    let path = format!("inputs/test_input_{}.txt", day);
    fs::read_to_string(&path).expect("Failed to read test input")
}

#[test]
fn d1_p1() {
    let input = read_test_input("day01");
    let result = days::day01::part1(&input);
    let expected = 11; // replace with actual expected output
    assert_eq!(result, expected);
}

#[test]
fn test_day01_part2() {
    let input = read_test_input("day01");
    let result = days::day01::part2(&input);
    let expected = 0; // Replace with actual expected output
    assert_eq!(result, expected);
}
