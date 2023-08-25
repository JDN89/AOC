// find complete overlap
// range rust
// https://doc.rust-lang.org/std/ops/struct.Range.html
// make 2 ranges -> rang1 and range 2
// range1.contains rang2.start && range2.end
//eazy peasy?
//
//steps: split at ,
//convert 2-3 to a tuple ;
//implement tryFrom for Range (value:(i32,i32)) -> Result<Self,Self::Error>

/* https://docs.rs/nom/latest/nom/combinator/fn.map_res.html */
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md
// https://docs.rs/nom/latest/nom/#traits
//nom use:
//map_res = convert to i32
// seperated pair -> to convert to range Ok((remiaing_input,start..end))
//
// fn parse_to_range(s: &str) -> Option<std::ops::Range<i32>>
// std lib looks easier in this case then using nom. lezz go
//
//implement nom also, just do both fo the fun of it

use std::ops::Range;

use itertools::Itertools;

fn parse_to_range(s: &str) -> Option<Range<i32>> {
    let pair: (i32, i32) = s
        .split('-')
        .map(|p| p.parse::<i32>().expect("Unable to parse str to i32"))
        .collect_tuple()
        .expect("didn't receive a pair in the String");
    Some(pair.0..pair.1)
}

pub fn process_part1(input: &str) -> u32 {
    let seperated_pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let pair: Option<(&str, &str)> = line.split(',').collect_tuple();
            let parsed_range = match pair {
                Some(p) => (parse_to_range(p.0), parse_to_range(p.1)),
                None => panic!("No pair found!"),
            };
        })
        .collect();

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), 2);
    }
    #[test]
    fn pare_to_range_test() {
        assert_eq!(parse_to_range("2-5"), Some(2..5));
    }

    // #[test]
    // fn test_day1_part2() {
    //     assert_eq!(process_part2(INPUT), 45000);
    // }
}
