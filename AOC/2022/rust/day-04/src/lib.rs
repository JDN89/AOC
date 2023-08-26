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

use std::ops::RangeInclusive;

use itertools::Itertools;

fn find_contains(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> u32 {
    if r1.contains(&r2.start()) && r1.contains(&r2.end())
        || r2.contains(&r1.start()) && r2.contains(&r1.end())
    {
        1
    } else {
        0
    }
}

//ok() converts result to an option
fn parse_to_range(s: &str) -> Option<RangeInclusive<i32>> {
    s.split('-')
        .map(|p| p.parse::<i32>().ok())
        .collect_tuple()
        .and_then(|(start, end)| Some(start?..=end?))
}

// single filter map
// We have an Option that contains a tuple (a, b), where a and b are themselves of type Option<RangeInclusive<i32>>.
// Inside the closure passed to and_then, we try to unpack the values inside a and b using the ? operator.
// If either a or b is None, the use of the ? operator will immediately result in the entire closure returning None.
pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .map(parse_to_range)
                .collect_tuple()
                .and_then(|(a, b)| Some(find_contains(a?, b?)))
        })
        .sum()
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
        assert_eq!(parse_to_range("2-5"), Some(2..=5));
    }

    #[test]
    fn find_overlap_test() {
        // assert_eq!(find_contains(2..7, 3..6), 1);
        // assert_eq!(find_contains(3..7, 3..6), 1);
        // assert_eq!(find_contains(3..4, 3..6), 1);
        assert_eq!(find_contains(6..=6, 4..=6), 1);
    }
    // #[test]
    // fn test_day1_part2() {
    //     assert_eq!(process_part2(INPUT), 45000);
    // }
}
