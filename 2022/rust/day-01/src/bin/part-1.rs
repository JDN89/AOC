use std::fs;
use day_01::process_part1;

fn main() {
    let total = fs::read_to_string("test_input.txt").unwrap();
    let _result = process_part1(&total);
}