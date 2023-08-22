use std::collections::HashSet;

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

fn convert_to_priority(ch: char) -> Option<u32> {
    //A starts at 65 -> 27
    //a starts at 97 _ 1
    // if ch is uppercase -> set bool
    // is_uppercase
    let upper = ch.is_uppercase();
    let priority: Option<u32>;
    if ch.is_ascii() {
        if upper {
            priority = Some(ch as u32 - 38)
        } else {
            priority = Some(ch as u32 - 96)
        }
    } else {
        priority = None
    }
    priority
}

pub fn process_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let _tup: (&str, &str) = split_in_middle(line);
            let _common = common_characters(_tup.0, _tup.1);
            let prio = convert_to_priority(_common).expect("not a valid character");
            prio
        })
        .sum();
    result
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

    #[test]
    fn test_convert_common_char_to_priority() {
        assert_eq!(convert_to_priority('a'), Some(1));
        assert_eq!(convert_to_priority('c'), Some(3));
        assert_eq!(convert_to_priority('A'), Some(27));
        assert_eq!(convert_to_priority('B'), Some(28));
    }

    // #[test]
    // fn test_day1_part2() {
    //     assert_eq!(process_part2(INPUT), 45000);
    // }
}
