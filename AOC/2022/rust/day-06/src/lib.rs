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

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn test_day6_part1() {
        assert_eq!(process_part1(INPUT), Some(10));
    }
}
