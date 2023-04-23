// addx += 2 cycles
// noop += 1 cycle
// 20 th cycle result
// 60th 100th adn 140th cycle result
//


use nom::{
    IResult,
    character::complete::char,
    character::complete::digit1,
    bytes::complete::tag,
    sequence::delimited,
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1
};

use nom::combinator::map_res;
use nom::sequence::{preceded, tuple};


fn main() {
    let lines = include_str!("../example.txt").lines();
    let mut cycle_check = 20;
        let mut cycle_count = 0;
        let mut x_register = 1;

    for line in lines {
        cycle_count +=1;
        println!("{:?}",line);
        if line.starts_with("addx") {
            let result = parse_instruction(line);
            println!("{:?}",result);
        }
    }

pub fn parse_instruction(ins: &str) -> IResult<&str, i32> {
    let instruction = preceded(tag("addx "), parse_signed_number);
    instruction(ins)
}

fn parse_signed_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((opt(char('-')), digit1)),
        |(sign, digits): (Option<char>, &str)| {
            let mut num = digits.parse::<i32>()?;
            if sign.is_some() {
                num = -num;
            }
            Ok(num)
        },
    )(input)
}





    }

