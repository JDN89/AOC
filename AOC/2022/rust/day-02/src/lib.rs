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
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Some(Ordering::Less),
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => Some(Ordering::Greater),
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
                Some(Equal) => 3 + moves[1].value(),
                Some(Ordering::Less) => 0 + moves[1].value(),
                None => panic!("unexpected cmp operation")
            }
        }).sum();
    result
}

//use the iter tools crate!!
pub fn process_part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "A Y
B X
C Z
A X
C Y";

    #[test]
    fn day1_part1() {
        assert_eq!(process_part1(INPUT), 21);
    }

    /* #[test]
    fn day1_part2() {
        assert_eq!(process_part2(INPUT), 45000);
    } */
}