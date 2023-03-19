use std::fmt;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};
use nom::bytes::complete::take_while1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;


