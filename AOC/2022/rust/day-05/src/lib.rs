// parse extern crate
// parse hole: 3 spaces
// parse crate and hole
// parse row num
// parse moves
//
use nom::branch::alt;
use nom::character::complete::alpha1;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    sequence::delimited,
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((tag("    "), delimited(char('['), alpha1, char(']'))))(input)?;

    let result = match c {
        "    " => None,
        value => Some(value),
    };
    Ok((input, result))
}

pub fn process_part1(input: &str) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), "CMZ");
    }
    #[test]
    fn test_parse_crate() {
        assert_eq!(parse_crate("    [A]"), Ok(("", Some("A"))));
    }
}

