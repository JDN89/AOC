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

pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let tup: (&str, &str) = split_in_middle(line);
        let common = common_characters(tup.0, tup.1);
    });

    todo!()
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
    fn day1_part1() {
        assert_eq!(process_part1(INPUT), 157);
    }

    #[test]
    fn test_split() {
        assert_eq!(split_in_middle("abcd"), ("ab", "cf"))
    }
    #[test]
    fn test_find_common_characters() {
        assert_eq!(common_characters("bart", "cord"), 'r')
    }

    /* #[test]
    fn day1_part2() {
        assert_eq!(process_part2(INPUT), 45000);
    } */
}
