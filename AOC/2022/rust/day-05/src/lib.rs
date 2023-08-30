// parse extern crate
// parse hole: 3 spaces
// parse crate and hole
// parse row num
// parse moves
//
//parse crate -> [A] Option crate
//parse hole -> "    " return none
//parse crate or hole -> option &char
//when all the vectors have the same lenght you can transpose them
//
//Transposing a matrix means flipping it over its diagonal, which in turn means swapping the row and column indices for each element. In other words, the element that was in the ith row and jth column of the original matrix will end up in the jth row and ith column of the transposed matrix.
//
use nom::branch::alt;
use nom::character::complete::{alpha1, multispace1};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::map,
    sequence::delimited,
    IResult,
};

todo!()
// how do i return nothing instead of an empty string?
fn parse_crate(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alpha1, char(']'))(input)
}

fn parse_hole(input: &str) -> IResult<&str, &str> {
    map(multispace1, |_| "")(input)
}
fn parse_crate_or_hole(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((parse_crate, parse_hole)))(input)
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
    const CRATE_OR_HOLE: &str = "    [D]    ";

    /* #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), "CMZ");
    } */

    #[test]
    fn test_parse_crate() {
        assert_eq!(parse_crate("[A]"), Ok(("", ("A"))));
    }
    #[test]
    fn test_parse_hole() {
        assert_eq!(parse_hole("    [A]"), Ok(("[A]", "")));
    }
    #[test]
    fn test_parse_crate_or_hole() {
        assert_eq!(
            parse_crate_or_hole("[N] [C]    "),
            Ok(("", vec!["N", "", "C", ""]))
        );
        assert_eq!(
            parse_crate_or_hole(CRATE_OR_HOLE),
            Ok(("", vec!["", "D", ""]))
        );
    }
}
