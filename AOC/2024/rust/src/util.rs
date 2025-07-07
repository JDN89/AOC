use std::{fs, path::PathBuf};
// I had issues with debugging in zed. so contsturct full path via cargo_manifest_dir
// env!("CARGO_MANIFEST_DIR") expands to the path of your project root at compile time.
pub fn read_input(path: &str) -> String {
    let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    fs::read_to_string(full_path).expect("Failed to read input")
}
// parse_lines<T>()
// split_groups()
// grid utilities
