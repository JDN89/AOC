use rust::days::day01; // import the day01 module
use rust::util::read_input;

#[test]
fn test_part1_example() {
    let input = read_input("inputs/test_input_day01.txt");

    assert_eq!(day01::part1(&input), 11);
    assert_eq!(day01::part2(&input), 31);
}
