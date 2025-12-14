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
fn d1_p2() {
    let input = read_test_input("day01");
    let result = days::day01::part2(&input);
    let expected = 31; // Replace with actual expected output
    assert_eq!(result, expected);
}

#[test]
fn d2_p1() {
    let input = read_test_input("day02");
    let result = days::day02::part1(&input);
    let expected = 2; // replace with actual expected output
    assert_eq!(result, expected);
}

#[test]
fn d2_p2() {
    let input = read_test_input("day02");
    let result = days::day02::part2(&input);
    let expected = 4; // Replace with actual expected output
    assert_eq!(result, expected);
}
