use nom::{bytes::complete::tag, character::complete::anychar, Finish, IResult, Parser, sequence::delimited};
use nom::branch::alt;
use nom::combinator::{all_consuming, map};
use nom::multi::{separated_list0, separated_list1};
use std::iter::Iterator;

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
    separated_list1(tag(" "), parse_crate_or_hole)(input)
}

pub fn receive_input(input: &str) {
    let crate_lines: Vec<_> = input.lines().by_ref()
        .map_while(|line| {
            all_consuming(parse_row)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    println!("{:?}", crate_lines);

    let crate_columns = transpose_rev(crate_lines);

    println!("{:?}", crate_columns);
}

// renamed to 👇 better indicate functionality
pub fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                // 👇
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_input() {
        let input = "FBFBBFFRLR";
        receive_input(&input);
    }

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