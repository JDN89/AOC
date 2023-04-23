// addx += 2 cycles
// noop += 1 cycle
// 20 th cycle result
// 60th 100th adn 140th cycle result
//
use nom::character::complete::char as nom_char; // Use an alias for 'char'

use nom::{
    IResult,
    character::complete::digit1,
    bytes::complete::tag,
    combinator::{opt, map_res},
};

use nom::sequence::{preceded, tuple};

fn main() {
    let lines = include_str!("../example.txt").lines();
    let mut cycle_check = 20;
    let mut cycle_count = 0;
    let mut x_register = 1;
    let mut signal_strength = 0;

    for line in lines {
        
        cycle_count += 1;
        if line.starts_with("addx") {
            cycle_count +=1;
            if cycle_count >= cycle_check {
                signal_strength += x_register * cycle_check;
                cycle_check += 40;
            }
            let result = parse_instruction(line);
            x_register += result.as_ref().unwrap().1;
        }
    }
    println!("result task 1: {:?}",signal_strength);
}

pub fn parse_instruction(ins: &str) -> IResult<&str, i32> {
    let mut instruction = preceded(tag("addx "), parse_signed_number);
    let (reminder, result) = instruction(ins)?;
    Ok((reminder, result))
}

fn parse_signed_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((opt(nom_char('-')), digit1)),
        |(sign, digits): (Option<char>, &str)| {
            let num = digits.parse::<i32>();
            match num {
                Ok(mut num) => {
                    if sign.is_some() {
                        num = -num;
                    }
                    Ok(num)
                }
                Err(e) => Err(e),
            }
        },
    )(input)
}

