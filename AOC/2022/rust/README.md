# Difficulty level:
Trivial: 1,2,4,6
Easy: 3,5,8,10,12,18,20,25
Medium: 7,9,11,13,14,15,21,23,24
Hard: 16,17,19,22

============
Very Easy: 1,2,3,6,25,13,9
Easy: 8,5,20,10,4,24,23,17,12
Medium: 11,21,14,18,7
Hard: 22,15,19,16

# Learned:

## Day 1:

- cargo run --bin part-1 command to run seperate bins
- iter tools crate for extra iteration functionality
    - [itertools](https://docs.rs/itertools/latest/itertools/index.html#)
- cargo watch -x check : see if program still compiles

## Day 2:

- CLI command: cp -r day-01/ day-02
    - coppy over the project in new folder
- Module Module std::cmp
    - utilities for comparing and ordering values
    - PartialOrd trait
- The FromStr trait in Rust is used to parse a string into some kind of object, and the parse method on strings utilizes the FromStr imp  lementation for the desired type to perform the parsing. 

## Day 4:
? on option value passes the wrapped value to the calling function and returns early if the wrapped value is None

## Day 5:
use nom:

 - [nom combinators] (https://docs.rs/nom/latest/nom/combinator/fn.map_res.html)
 - [choosing a combinator] (https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md)
 - [traits] (https://docs.rs/nom/latest/nom/#traits)

