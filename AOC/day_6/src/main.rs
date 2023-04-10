use crate::find_marker::find_marker;

mod find_marker;

fn main() {
    dbg!(find_marker(include_str!("input.txt")));
}
