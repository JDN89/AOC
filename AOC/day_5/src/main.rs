use nom::combinator::all_consuming;
use nom::Finish;

use crate::parse::{Instruction, parse_instruction, parse_row};
use crate::transpose::transpose_rev;

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
    let mut crate_columns = transpose_rev(crate_lines);
    for col in &crate_columns {
        println!("{:?}", col);
    }

    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<Instruction> = lines
        .map(|line|
            all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for ins in &instructions {
            for p in 0..ins.mve {
               let from  = &crate_columns[ins.from];
                let to = &crate_columns[ins.to];
                // println!("from: {:?}, to: {:?}",from,to)
               let temp =  crate_columns[ins.from].pop();
                println!("temp: {:?} ",temp);
                match temp {
                    Some(T) => crate_columns[ins.to].push(T),
                    None => println!("stack is empty")
                }

            }
        }
    println!("result: {:?}", crate_columns);
    }