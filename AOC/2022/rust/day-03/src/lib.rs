
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Priority(u32);

impl TryFrom<char> for Priority {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'a'..='z' => Ok(Priority(ch as u32 - 96)),
            'A'..='Z' => Ok(Priority(ch as u32 - 38)),
            _ => Err("Character is not a valid alphabet letter"),
        }
    }
}

fn split_in_middle(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    let mut split_at = mid;
    while !s.is_char_boundary(split_at) && split_at < s.len() {
        split_at += 1;
    }
    s.split_at(split_at)
}

fn common_characters(s1: &str, s2: &str) -> char {
    let set1: HashSet<_> = s1.chars().collect();
    for ch in s2.chars() {
        if set1.contains(&ch) {
            return ch;
        }
    }
    panic!("no common characters")
}

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (part1, part2) = split_in_middle(line);
            let common_char = common_characters(part1, part2);
            Priority::try_from(common_char)
                .expect("Not a valid character")
                .0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), 157);
    }

    #[test]
    fn test_split_in_middle() {
        assert_eq!(split_in_middle("abcd"), ("ab", "cd"));
    }

    #[test]
    fn test_common_characters_in_strings() {
        assert_eq!(common_characters("bart", "cord"), 'r');
    }

    // #[test]
    // fn test_day1_part2() {
    //     assert_eq!(process_part2(INPUT), 45000);
    // }
}
