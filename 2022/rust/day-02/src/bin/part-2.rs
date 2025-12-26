use day_02::process_part2;
use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let _result = process_part2(&lines);
}
