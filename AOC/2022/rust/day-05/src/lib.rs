pub fn process_part1(input: &str) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), "CMZ");
    }
}
