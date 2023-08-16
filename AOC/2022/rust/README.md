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

start with enum move
Rock
paper Siscors

round theirs ours

partial ord
        If the input is "A" or "X", return Move::Rock.
    If the input is "B" or "Y", return Move::Paper.
    If the input is "C" or "Z", return Move::Scissors.
    For any other input, return an error.

    fromtstr

        Scissors are less than Rock (i.e., Rock wins over Scissors).
    Rock is greater than Scissors.
    For all other comparisons, it falls back to comparing their numeric representation (1 for Rock, 2 for Paper, 3 for Scissors).
