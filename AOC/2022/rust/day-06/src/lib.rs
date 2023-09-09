// find first 4 distinct characters give the last char in the set
// work with windowslices and hashset?

use std::collections::HashSet;

pub fn process_part1(input: &str) -> Option<usize> {
    for (i, window) in input.chars().collect::<Vec<_>>().windows(4).enumerate() {
        let unique_characters: HashSet<_> = window.iter().collect();
        if unique_characters.len() == 4 {
            return Some(i + 4);
        }
    }
    None
}

pub fn process_part1_other_solution(i: &str) -> Option<usize> {
    i.as_bytes()
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
        .map(|position| position + 4)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{process_part1, process_part1_other_solution};
    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]

    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(Some(index), process_part1(input))
    }

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker_v2(index: usize, input: &str) {
        assert_eq!(Some(index), process_part1_other_solution(input))
    }
}
