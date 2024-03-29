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

- When I use map, I should `collect` the result or otherwise the iterator won't get CONSUMED
    - Iterators are lazy, meaning the compuation won't be preformed until you consume the iterator with `.collect()`
- An alternitive is using the for each method or a for loop

## Day 6:
- .iter().collect() -> `iter()` yields all the elements from the slice -> `collect()` to collect into a new data strucutre
- cargo install cargo-nextest: install better testing tool as binary on my system. now `cargo-nextest` is a cli tool and command
    - [cargo nextest syntax] (https://github.com/frondeus/test-case/wiki/Syntax)
- as_bytes()
    - Character level (Unicode scalar values): This is when you're interested in processing individual characters, taking into account their Unicode representation. This might be important when you're handling text data that needs to be interpreted according to its meaning in human languages. In Rust, you can iterate over a string's characters using the .chars() method.
    - Byte level: This is when you're interested in processing the raw byte representation of the string, without any special consideration for Unicode scalar values. This approach can be useful when you're dealing with data that's primarily ASCII (where each character is one byte) or when you're interested in the byte-by-byte layout of the string in memory, irrespective of its meaning in human languages. In Rust, you can get the byte representation of a string using the .as_bytes() method.

## Day 8:
- learned about dereferencing: dereference a ref to assign it's underlying value to a new var;
- chatgpt: imagine having a book
    - A reference (&T) is like having a bookmark in the book, it tells you where the content is, but it isn't the content itself.
    - Dereferencing (*): opening the book at the bookmark and reading the contents
- .iter().enumerate(): called on iterators and return iter that gives you tuple (index,val) for each item in the original iterator.   
- found an article that explains how to bench mark your tests
    - [blog explain bench marks] (https://seenaburns.com/benchmarking-rust-with-cargo-bench)
    - [rust docs] (https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
    - I'm using some nightly features so to run the bench mark:  rustup run nightly cargo bench (no need to swithc to nightly)
- could no longer cargo run because of: #![feature(test)] -> switch to nightly
    - rustup install nightly
    - rustup override set nightly
    - cargo run || cargo bench || cargo test
    - rustup override set stable -> revert to stable verison
    - rustup update nightly 
- possible optimizations:
    - count inside iter fn how many booleans are true?
    - don't return visible trees from iter functions -> outer scope no need to return
    - set outer boundry of grid to true in seperate function, instead of 4 times in iter funcitons
    - enjoy



use nom:

 - [nom combinators] (https://docs.rs/nom/latest/nom/combinator/fn.map_res.html)
 - [choosing a combinator] (https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md)
 - [traits] (https://docs.rs/nom/latest/nom/#traits)

