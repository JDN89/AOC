use nom::combinator::all_consuming;
use nom::Finish;

use crate::parse::{parse_instruction, parse_row, Instruction};
use crate::transpose::transpose_rev;

mod parse;
mod transpose;

fn main() {
    let mut lines = include_str!("input.txt").lines();

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
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for ins in &instructions {
            let mut temp_crate: Vec<char> = Vec::new();
        for _p in 0..ins.mve {

            let temp = crate_columns[ins.from].pop();

            println!("temp: {:?} ", temp);
            match temp {
                Some(c) => temp_crate.push(c),
                //Some(t) => crate_columns[ins.to].push(t),
                None => println!("stack is empty"),
            }
        }
        temp_crate.reverse();

        for item in temp_crate {
            crate_lines[ins.to].push(item);
        }
        
        
    }
    println!("result: {:?}", crate_columns);
    let mut result: Vec<&char> = vec![];
    let _ = crate_columns.iter().for_each(|vec| match vec.last() {
        Some(last_value) => result.push(last_value),
        //println!("last: {:?}",last_value) ,
        None => println!("no value"),
    });
    let mut answer = String::new();
    for ch in result {
        answer.push(*ch);
    }
    println!("result = {:?}", answer);
}
