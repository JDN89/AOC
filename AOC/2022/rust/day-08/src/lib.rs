pub fn process_part1(input: &str) -> Option<usize> {
    let temp: Vec<_> = input.lines().collect();
    dbg!(temp);
    Some(3)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::process_part1;
    #[test_case(
        21,
        "30373
25512
65332
33549
35390"
    )]
    fn find_visible_trees(index: usize, input: &str) {
        assert_eq!(Some(index), process_part1(input))
    }
}
