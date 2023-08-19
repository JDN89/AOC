// run specific part with: cargo build --bin part-1 or part-2
// A  X ROCK    1
// B  Y PAPER   2
// C  Z SICCORS 3

// start with enum move
// Rock
// paper Siscors
//
// round theirs ours
//
// partial ord
// If the input is "A" or "X", return Move::Rock.
// If the input is "B" or "Y", return Move::Paper.
// If the input is "C" or "Z", return Move::Scissors.
// For any other input, return an error.
//
// fromtstr
//
// Scissors are less than Rock (i.e., Rock wins over Scissors).
// Rock is greater than Scissors.
// For all other comparisons, it falls back to comparing their numeric representation (1 for Rock, 2 for Paper, 3 for Scissors).

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Move::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Some(Ordering::Greater),
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => Some(Ordering::Less),
            //check if the same through partialEq
            _ if self == other => Some(Equal),
            _ => None
        }
    }
}

impl Move {
    fn value(self) -> u32 {
        self as u32
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("This is not a move")),
        }
    }
}

pub fn process_part1(input: &str) -> u32 {
    let result = input.lines()
        .map(|line| {
            let moves: Vec<Move> = line.
                split(" ").map(
                |m| m.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Greater) => 6 + moves[1] as u32,
                Some(Ordering::Equal) => 3 + moves[1].value(),
                Some(Ordering::Less) => 0 + moves[1].value(),
                None => panic!("unexpected cmp operation")
            }
        }).sum();
    result
}

//use the iter tools crate!!
pub fn process_part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn day1_part1() {
        assert_eq!(process_part1(INPUT), 15);
    }

    /* #[test]
    fn day1_part2() {
        assert_eq!(process_part2(INPUT), 45000);
    } */
}