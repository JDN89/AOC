use nom::combinator::all_consuming;
use nom::Finish;
use crate::parse::{parse_row, transpose_rev};

mod parse;

fn main() {
    let mut lines = include_str!("test.txt").lines();

    let crate_lines: Vec<_> = lines
        // 👇
        .by_ref()
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
    //todo parse pile numbers
    //todo parse empty line
    //todo parse instructions and put into struct

    // we've consumed the "numbers line" but not the separating line
    // assert!(lines.next().unwrap().is_empty());
}
