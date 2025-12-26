use day_06::process_part1;
use std::fs;

fn main() {
    println!("inside main");
    let total = fs::read_to_string("input.txt").unwrap();
    let result = process_part1(&total);
    println!("{:?}", result);
}
