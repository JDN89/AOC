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
//I don't think i need to parse the crate numbers, I can use the indexes
//
//I need to parse the instrucations tough
use nom::branch::alt;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::{
    bytes::complete::tag, character::complete::char, combinator::map, sequence::delimited, IResult,
};

#[derive(Debug, PartialEq)]
struct MoveOperation {
    amount: usize,
    from: usize,
    to: usize,
}

impl TryFrom<(usize, usize, usize)> for MoveOperation {
    type Error = String;

    fn try_from(tuple: (usize, usize, usize)) -> Result<Self, Self::Error> {
        let (amount, from, to) = tuple;
        Ok(MoveOperation { amount, from, to })
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

// -1 because we use indexes
fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_number, |n| n - 1)(input)
}

fn parse_move_operation(i: &str) -> IResult<&str, Result<MoveOperation, String>> {
    let parser = tuple((
        preceded(tag("move "), parse_number),
        preceded(tag(" from "), parse_pile_number),
        preceded(tag(" to "), parse_pile_number),
    ));
    map(parser, |tuple| MoveOperation::try_from(tuple))(i)
}

// how do i return nothing instead of an empty string?
fn parse_crate(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alpha1, char(']'))(input)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<&str>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(input)
}

// crates and holes are seperated by single spaces
// so now we have to parse a crate or a hole and discard the single spaces (remaining input)
// and collect the crates in a vector
// . These spaces are "consumed" in the sense that the parser moves past them during its operation, but they are not included in the final output of the function.
fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

// write transpose function

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

    #[test]
    fn test_parse_crate() {
        assert_eq!(parse_crate("[A]"), Ok(("", ("A"))));
    }

    #[test]
    fn test_parse_hole() {
        assert_eq!(parse_hole("   "), Ok(("", ())));
        assert_eq!(parse_hole("    "), Ok((" ", ())));
    }
    #[test]
    fn test_parse_crate_or_hole() {
        assert_eq!(parse_crate_or_hole("[A]"), Ok(("", Some("A"))));
        assert_eq!(parse_crate_or_hole("   "), Ok(("", None)));
    }
    #[test]
    fn test_parse_crate_line() {
        assert_eq!(
            parse_crate_line("[A] [B]     [C]"),
            Ok(("", vec![Some("A"), Some("B"), None, Some("C")]))
        );
        assert_eq!(parse_crate_line("[A]    "), Ok(("", vec![Some("A"), None])));
        assert_eq!(
            parse_crate_line("[A]     [B]"),
            Ok(("", vec![Some("A"), None, Some("B")]))
        );
        assert_eq!(parse_crate_line("    [A]"), Ok(("", vec![None, Some("A")])));
    }
    #[test]
    fn test_parse_move_operation() {
        assert_eq!(
            parse_move_operation("move 1 from 2 to 1"),
            Ok((
                "",
                Ok(MoveOperation {
                    amount: 1,
                    from: 1,
                    to: 0
                })
            ))
        );
    }
}