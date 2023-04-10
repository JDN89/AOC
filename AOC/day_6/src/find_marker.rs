use std::collections::HashSet;

pub fn find_marker(input:&str) -> Option<usize>  {
    input.as_bytes()
        .windows(14)
        .position(|window| window.iter().collect::<HashSet<_>>().len()==14)
        .map(|pos| pos + 14)
}

#[cfg(test)]
mod tests {
    use super::find_marker;

    #[test]
    fn test_find_marker() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = find_marker(input);
        assert_eq!(result, Some(5));
    }
}
