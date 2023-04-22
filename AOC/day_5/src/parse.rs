use nom::{
    IResult,
    character::complete::anychar,
    bytes::complete::tag,
    sequence::delimited,
    branch::alt,
    combinator::{map},
    multi::separated_list1
};
use std::iter::Iterator;
use std::num::ParseIntError;
use nom::bytes::streaming::take_while1;
use nom::character::is_digit;
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::tuple;

pub fn parse_crate(input: &str) -> IResult<&str, char> {
    delimited(tag("["), anychar, tag("]"))(input)
}

pub fn parse_hole(input: &str) -> IResult<&str, ()> {
    let (reminder, _) = tag("   ")(input)?;
    Ok((reminder,()))
}

pub fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<char>> {
    let parse_crate_some = map(parse_crate, Some);
    let parse_hole_none = map(parse_hole, |_| None);
    alt((parse_crate_some, parse_hole_none))(input)
}

pub fn parse_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (reminder,row) = separated_list1(tag(" "), parse_crate_or_hole)(input)?;
    println!("reminder: {:?}",reminder);
    Ok((reminder,row))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

pub fn pile_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), parse_number)(input)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crate_success() {
        let input = "[A]";
        let result = parse_crate(input);
        assert_eq!(result, Ok(("", 'A')));
    }

    #[test]
    fn test_parse_hole() {
        let input = "   ";
        let result = parse_hole(input);
        assert_eq!(result, Ok(("", ())))
    }

    #[test]
    fn test_parse_crate_or_hole_crate() {
        let input = "[A]";
        let result = parse_crate_or_hole(input);
        assert_eq!(result, Ok(("", Some('A'))));
    }

    #[test]
    fn test_parse_crate_or_hole_hole() {
        let input = "   ";
        let result = parse_crate_or_hole(input);
        assert_eq!(result, Ok(("", None)));
    }

    #[test]
    fn test_parse_crate_or_hole_invalid() {
        let input = "[AB]";
        let result = parse_crate_or_hole(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_row() {
        let input = "[A] [B] [C]";
        let result = parse_row(input);
        assert_eq!(result, Ok(("", vec![Some('A'), Some('B'), Some('C')])));
    }

    #[test]
    fn test_parse_row_hole_and_crate() {
        //5 spaces,between [A] and [B], " " switches between crate and hole so
        // "   " is a hole and "[A]" is a crate
        let input = "[A]     [B]    ";
        let result = parse_row(input);
        assert_eq!(result, Ok(("", vec![Some('A'), None, Some('B'), None])));
    }
}