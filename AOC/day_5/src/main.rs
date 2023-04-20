use nom::combinator::all_consuming;
use nom::Finish;
use crate::parse::{parse_row, receive_input, transpose_rev};

mod parse;

fn main() {
    let mut lines = include_str!("test.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_row)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let crate_columns = transpose_rev(crate_lines);
    for col in &crate_columns {
        println!("{col:?}");
    }
}
