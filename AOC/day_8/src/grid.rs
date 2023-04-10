use std::error::Error;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::IResult;
use nom::multi::separated_list1;

// put in grid
// loop over rows and collumns
// for y in trees .len
// for x in trees[0].len
// then rev


// for tomorrow-> convert char to digit

//TODO: check error handling in from zero to production
#[derive(Debug,PartialEq)]
enum MyError {
    InvalidDigit(char),
}

pub fn to_num(c: char) -> Result<u32, MyError> {
    match c.to_digit(10) {
        Some(value) => Ok(value),
        None => Err(MyError::InvalidDigit(c)),
    }
}

pub fn create_grid(input: &str) -> Option<Vec<u32>> {
    for line in input.lines() {
//TODO: do something with seperated_list and new line and parser to num
        //check function parse crates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
   fn convert_char_to_num() {
        let input = '4';
        let result = to_num(input);
        assert_eq!(result, Ok(4))
    }
}