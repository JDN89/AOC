use std::fs::{self};

/// Read the entire file into a single String
pub fn read_input(path: &str) -> String {
    let input = fs::read_to_string(path).expect("Failed to read file");
    input
}
