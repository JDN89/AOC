use day_02::process_part1;
use std::fs;

fn main() {
    let total = fs::read_to_string("test_input.txt").unwrap();
    let _result = process_part1(&total);
}
