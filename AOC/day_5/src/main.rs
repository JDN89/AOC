use nom::combinator::all_consuming;
use nom::Finish;
use crate::parse::{parse_row};
use crate::transpose::{transpose_rev};

mod parse;
mod transpose;

fn main() {
    let mut lines = include_str!("test.txt").lines();

    let crate_lines: Vec<_> = lines
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
        println!("{:?}", col);
    }

    for line in lines {
        println!("{:?}", line);
    }
    // let number_lines: Vec<_> = lines
    //     .map(|line| {
    //         all_consuming(pile_numbers)(line)
    //             .finish()
    //             .ok()
    //             .map(|(_, line)| line)
    //     })
    //     .collect();
    //
    // for line in &number_lines {
    //     println!("{:?}", line);
    // }
}