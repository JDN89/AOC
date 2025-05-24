use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read input")
}
// parse_lines<T>()
// split_groups()
// grid utilities
