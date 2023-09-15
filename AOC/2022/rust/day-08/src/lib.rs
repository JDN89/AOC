pub fn process_part1(input: &str) -> Option<u32> {
    let temp: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a character"))
                .collect::<Vec<u32>>()
        })
        .collect();

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
    fn find_visible_trees(visible_trees: u32, input: &str) {
        assert_eq!(Some(visible_trees), process_part1(input))
    }
}
