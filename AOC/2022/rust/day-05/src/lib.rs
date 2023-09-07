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
use nom::combinator::{all_consuming, map_res};
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
// implement clone trait
fn transpose<T: Clone>(input: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    let rows = input.len();
    let cols = input[0].len();

    let mut transposed = Vec::new();

    for j in 0..cols {
        let mut new_row = Vec::new();
        for i in 0..rows {
            //check if input[i] [j] contains a Some(value) -> filter out none and push the values
            if let Some(value) = input[i][j].as_ref() {
                new_row.push(value.clone());
            }
        }
        transposed.push(new_row);
    }

    transposed
}

pub fn process_part1(input: &str) -> String {
    let mut crate_lines = vec![];

    input.lines().for_each(|line| {
        // if _rest,crate line is result of parser ->execute code block
        if let Ok((_rest, crate_line)) = all_consuming(parse_crate_line)(line) {
            crate_lines.push(crate_line);
        }
    });

    let mut transposed_crate_stacks: Vec<Vec<&str>> = transpose(crate_lines)
        .into_iter()
        .map(|mut stack| {
            stack.reverse();
            stack
        })
        .collect();

    // Parse move instructions
    let move_instructions: Vec<_> = input
        .lines()
        // closure provided to filter map return Option -> we keep the value from some and filter out None
        .filter_map(|line| {
            all_consuming(parse_move_operation)(line)
                // ok to convert Result to Option!!
                .ok()
                .map(|(_rest, instr)| instr)
        })
        .collect();

    for mve in move_instructions {
        dbg!(&mve);
        match mve {
            Ok(mve) => {
                for _ in 0..mve.amount {
                    if let Some(cr) = transposed_crate_stacks[mve.from].pop() {
                        // dbg!(cr);
                        transposed_crate_stacks[mve.to].push(cr);
                    } else {
                        println!("nothing here");
                        break;
                    }
                }
            }
            _ => (),
        }
    }
    let mut result = String::new();
    for stack in &mut transposed_crate_stacks {
        if let Some(crate_str) = stack.pop() {
            result.push_str(crate_str);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(INPUT), ("CMZ"));
    }

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

    #[test]
    fn test_transpose() {
        let matrix: Vec<Vec<Option<i32>>> = vec![
            vec![Some(1), Some(2), Some(3)],
            vec![Some(4), Some(5), Some(6)],
            vec![Some(7), Some(8), Some(9)],
        ];

        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

        assert_eq!(transpose(matrix), expected);
    }
}
