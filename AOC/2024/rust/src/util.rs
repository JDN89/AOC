use std::fs::{self};

/// Read the entire file into a single String
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file at path: {}", path))
}
